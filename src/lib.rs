use bevy::{
    app::{App, Plugin},
    asset::{load_internal_asset, Asset, Handle},
    pbr::{ExtendedMaterial, MaterialExtension, MaterialPlugin, StandardMaterial},
    reflect::{std_traits::ReflectDefault, Reflect},
    render::{
        render_resource::{AsBindGroup, Shader, ShaderRef, ShaderType},
        texture::Image,
    },
};

static SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(11427442657628709095);

pub struct GltfPbrExtrasPlugin;

#[derive(Clone, Asset, AsBindGroup, Default, Reflect, Debug)]
#[reflect(Default)]
pub struct GltfPbrExtension {
    #[uniform(100)]
    pub pbr_extension_data: GltfPbrExtensionData,
    #[texture(101)]
    #[sampler(102)]
    pub iridescence_texture: Option<Handle<Image>>,
    #[texture(103)]
    #[sampler(104)]
    pub iridescence_thickness_texture: Option<Handle<Image>>,
}

#[derive(Clone, ShaderType, Reflect, Debug)]
pub struct GltfPbrExtensionData {
    pub iridescence_factor: f32,
    pub iridescence_ior: f32,
    pub iridescence_thickness_minimum: f32,
    pub iridescence_thickness_maximum: f32,
}

pub type GltfPbrExtendedMaterial = ExtendedMaterial<StandardMaterial, GltfPbrExtension>;

impl Plugin for GltfPbrExtrasPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            SHADER_HANDLE,
            "gltf_pbr_extended_material.wgsl",
            Shader::from_wgsl
        );

        app.register_type::<GltfPbrExtension>()
            .register_type::<GltfPbrExtensionData>()
            .add_plugins(MaterialPlugin::<GltfPbrExtendedMaterial>::default());
    }
}

impl MaterialExtension for GltfPbrExtension {
    fn fragment_shader() -> ShaderRef {
        SHADER_HANDLE.clone().into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        SHADER_HANDLE.clone().into()
    }
}

impl Default for GltfPbrExtensionData {
    fn default() -> Self {
        // Values are from the spec.
        Self {
            iridescence_factor: 0.0,
            iridescence_ior: 1.3,
            iridescence_thickness_minimum: 100.0,
            iridescence_thickness_maximum: 400.0,
        }
    }
}
