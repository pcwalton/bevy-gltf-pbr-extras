// bevy-gltf-pbr-extras/examples/iridescence-suzanne.rs

use bevy::{
    core_pipeline::{tonemapping::Tonemapping, Skybox},
    math::vec3,
    prelude::*,
};
use bevy_gltf_pbr_extras::{
    GltfPbrExtendedMaterial, GltfPbrExtension, GltfPbrExtensionData, GltfPbrExtrasPlugin,
};
use light_consts::lux::{self, AMBIENT_DAYLIGHT, FULL_DAYLIGHT};

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::BLACK,
            brightness: 0.0,
        })
        .add_plugins(DefaultPlugins)
        .add_plugins(GltfPbrExtrasPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, brighten_light)
        .add_systems(Update, add_iridescent_materials)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(2.0, 1.0, 8.0).looking_at(vec3(0.0, -0.2, 0.0), Vec3::Y),
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::AcesFitted,
            ..default()
        })
        .insert(Skybox {
            brightness: 5000.0,
            image: asset_server.load("pisa_specular_rgb9e5_zstd.ktx2"),
        })
        .insert(EnvironmentMapLight {
            diffuse_map: asset_server.load("pisa_diffuse_rgb9e5_zstd.ktx2"),
            specular_map: asset_server.load("pisa_specular_rgb9e5_zstd.ktx2"),
            intensity: 3000.0,
        });

    /*commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(2.0, 1.0, 8.0).looking_at(vec3(0.0, -0.2, 0.0), Vec3::Y),
        ..default()
    });*/

    commands.spawn(SceneBundle {
        scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("IridescenceSuzanne.gltf")),
        ..default()
    });
}

fn brighten_light(mut lights: Query<&mut DirectionalLight>) {
    for mut light in lights.iter_mut() {
        light.illuminance = lux::OVERCAST_DAY;
    }
}

fn add_iridescent_materials(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    nodes: Query<(Entity, &Name, &Handle<StandardMaterial>)>,
    standard_material_assets: Res<Assets<StandardMaterial>>,
    mut gltf_pbr_material_assets: ResMut<Assets<GltfPbrExtendedMaterial>>,
) {
    for (entity, name, standard_material_handle) in nodes.iter() {
        let Some(standard_material) = standard_material_assets.get(standard_material_handle) else {
            continue;
        };

        #[allow(clippy::if_same_then_else)]
        let gltf_pbr_extension = if **name == *"Suzanne1" {
            GltfPbrExtension {
                pbr_extension_data: GltfPbrExtensionData {
                    iridescence_factor: 1.0,
                    iridescence_ior: 1.33,
                    ..default()
                },
                ..default()
            }
        } else if **name == *"Suzanne2" {
            GltfPbrExtension {
                pbr_extension_data: GltfPbrExtensionData {
                    iridescence_factor: 1.0,
                    iridescence_ior: 1.33,
                    ..default()
                },
                ..default()
            }
        } else if **name == *"Suzanne3" {
            GltfPbrExtension {
                // TODO: texture
                pbr_extension_data: GltfPbrExtensionData {
                    iridescence_factor: 1.0,
                    iridescence_ior: 1.8,
                    //iridescence_ior: 1.33,
                    iridescence_thickness_minimum: 200.0,
                    iridescence_thickness_maximum: 800.0,
                    //iridescence_thickness_maximum: 400.0,
                },
                iridescence_thickness_texture: Some(asset_server.load("noise.png")),
                ..default()
            }
        } else {
            continue;
        };

        commands
            .entity(entity)
            .remove::<Handle<StandardMaterial>>()
            .insert(gltf_pbr_material_assets.add(GltfPbrExtendedMaterial {
                base: StandardMaterial {
                    custom_fresnel: true,
                    ..standard_material.clone()
                },
                extension: gltf_pbr_extension,
            }));
    }
}
