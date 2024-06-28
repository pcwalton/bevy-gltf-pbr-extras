use bevy::{
    app::{App, Plugin},
    asset::{load_internal_asset, Asset, Handle},
    pbr::{
        ExtendedMaterial, MaterialExtension, MaterialExtensionKey, MaterialExtensionPipeline,
        MaterialPlugin, StandardMaterial,
    },
    reflect::{std_traits::ReflectDefault, Reflect},
    render::{
        mesh::MeshVertexBufferLayoutRef,
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, Shader, ShaderRef, ShaderType,
            SpecializedMeshPipelineError,
        },
        texture::Image,
    },
};

static SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(11427442657628709095);

pub struct GltfPbrExtrasPlugin;

#[derive(Clone, Asset, AsBindGroup, Default, Reflect, Debug)]
#[reflect(Default)]
#[bind_group_data(GltfPbrExtensionKey)]
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

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct GltfPbrExtensionKey {
    has_iridescence_texture: bool,
    has_iridescence_thickness_texture: bool,
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

    fn specialize(
        _: &MaterialExtensionPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        _: &MeshVertexBufferLayoutRef,
        key: MaterialExtensionKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        if let Some(ref mut fragment_state) = descriptor.fragment {
            if key.bind_group_data.has_iridescence_texture {
                fragment_state
                    .shader_defs
                    .push("HAS_IRIDESCENCE_TEXTURE".into());
            }
            if key.bind_group_data.has_iridescence_thickness_texture {
                fragment_state
                    .shader_defs
                    .push("HAS_IRIDESCENCE_THICKNESS_TEXTURE".into());
            }
        }

        Ok(())
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

impl From<&GltfPbrExtension> for GltfPbrExtensionKey {
    fn from(value: &GltfPbrExtension) -> Self {
        GltfPbrExtensionKey {
            has_iridescence_texture: value.iridescence_texture.is_some(),
            has_iridescence_thickness_texture: value.iridescence_thickness_texture.is_some(),
        }
    }
}
