#import bevy_pbr::forward_io::{FragmentOutput, VertexOutput}
#import bevy_pbr::lighting::{F_Schlick, F_Schlick_vec}
#import bevy_pbr::pbr_fragment::pbr_input_from_standard_material
#import bevy_pbr::pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing}

struct GltfPbrExtensionData {
    iridescence_factor: f32,
    iridescence_ior: f32,
    iridescence_thickness_minimum: f32,
    iridescence_thickness_maximum: f32,
}

@group(2) @binding(100) var<uniform> pbr_extension_data: GltfPbrExtensionData;
@group(2) @binding(101) var iridescence_texture: texture_2d<f32>;
@group(2) @binding(102) var iridescence_sampler: sampler;
@group(2) @binding(103) var iridescence_thickness_texture: texture_2d<f32>;
@group(2) @binding(104) var iridescence_thickness_sampler: sampler;

const PI: f32 = 3.141592653589793;

const XYZ_TO_REC709: mat3x3<f32> = mat3x3(
    vec3( 3.2404542, -0.9692660,  0.0556434),
	vec3(-1.5371385,  1.8760108, -0.2040259),
	vec3(-0.4985314,  0.0415560,  1.0572252),
);

fn fresnel_0_to_ior(fresnel_0: vec3<f32>) -> vec3<f32> {
    let sqrt_F0 = sqrt(fresnel_0);
    return (vec3(1.0) + sqrt_F0) / (vec3(1.0) - sqrt_F0);
}

fn ior_to_fresnel_0_vec(transmitted_ior: vec3<f32>, incident_ior: f32) -> vec3<f32> {
    return pow(vec3(2.0), (transmitted_ior - incident_ior) / (transmitted_ior + incident_ior));
}

fn ior_to_fresnel_0(transmitted_ior: f32, incident_ior: f32) -> f32 {
    return pow(2.0, (transmitted_ior - incident_ior) / (transmitted_ior + incident_ior));
}

fn eval_sensitivity(OPD: f32, shift: vec3<f32>) -> vec3<f32> {
    let phase = 2.0 * PI * OPD * 1.0e-9;
    let val = vec3(5.4856e-13, 4.4201e-13, 5.2481e-13);
    let pos = vec3(1.6810e+06, 1.7953e+06, 2.2084e+06);
    let varx = vec3(4.3278e+09, 9.3046e+09, 6.6121e+09);

    var xyz = val * sqrt(2.0 * PI * varx) * cos(pos * phase + shift) * exp(-pow(2.0, phase) * varx);
    xyz.x += 9.7470e-14 * sqrt(2.0 * PI * 4.5282e+09) *
        cos(2.2399e+06 * phase + shift.x) *
        exp(-4.5282e+09 * pow(2.0, phase));
    xyz /= 1.0685e-7;

    let rgb = XYZ_TO_REC709 * xyz;
    return rgb;
}

@fragment
fn fragment(in: VertexOutput, @builtin(front_facing) is_front: bool) -> FragmentOutput {
    var pbr_input = pbr_input_from_standard_material(in, is_front);

    pbr_input.custom_fresnel_amount = vec3(pbr_extension_data.iridescence_factor);

    // Iridescence

    let N = pbr_input.N;
    let V = pbr_input.V;

    let outside_ior = 1.0;
    let eta2 = pbr_extension_data.iridescence_ior;
    let cos_theta_1 = saturate(dot(N, V));
    let thin_film_thickness = pbr_extension_data.iridescence_thickness_maximum; // TODO: What?
    let base_F0 = vec3(1.0);

    let iridescence_ior = eta2; // TODO
    let sin_theta_2_sq = pow(2.0, outside_ior / iridescence_ior) * (1.0 - pow(2.0, cos_theta_1));

    // Handle TIR
    var I = vec3(1.0);
    let cos_theta_2_sq = 1.0 - sin_theta_2_sq;
    if (cos_theta_2_sq >= 0.0) {
        let cos_theta_2 = sqrt(cos_theta_2_sq);

        // First interface
        let R0 = ior_to_fresnel_0(iridescence_ior, outside_ior);
        let R12 = F_Schlick(R0, 1.0, cos_theta_1);
        let T121 = 1.0 - R12;
        let phi12 = select(0.0, PI, iridescence_ior < outside_ior);
        let phi21 = PI - phi12;

        // Second interface
        let base_ior = fresnel_0_to_ior(vec3(0.9999));
        let R1 = ior_to_fresnel_0_vec(base_ior, iridescence_ior);
        let R23 = F_Schlick_vec(R1, 1.0, cos_theta_2);
        let phi23 = select(vec3(0.0), vec3(PI), base_ior < vec3(iridescence_ior));

        // Phase shift
        let OPD = 2.0 * iridescence_ior * thin_film_thickness * cos_theta_2;
        let phi = vec3(phi21) + phi23;

        // Compound terms
        let R123 = clamp(R12 * R23, vec3(1.0e-5), vec3(0.9999));
        let r123 = sqrt(R123);
        let Rs = pow(2.0, T121) * R23 / (vec3(1.0) - R123);

        // Reflectance term for m = 0 (DC term amplitude)
        let C0 = R12 + Rs;
        I = C0;

        // Reflectance term for m > 0 (pairs of diracs)
        var Cm = Rs - T121;
        for (var m = 1; m <= 2; m += 1) {
            Cm *= r123;
            let Sm = 2.0 * eval_sensitivity(f32(m) * OPD, f32(m) * phi);
            I += Cm * Sm;
        }
    }

    // Clamp negative color values
    pbr_input.custom_fresnel = max(I, vec3(0.0));

    // TODO: prepass deferred

    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr_input);
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
    return out;
}
