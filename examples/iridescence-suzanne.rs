// bevy-gltf-pbr-extras/examples/iridescence-suzanne.rs

use bevy::{math::vec3, prelude::*};
use bevy_gltf_pbr_extras::{
    GltfPbrExtendedMaterial, GltfPbrExtension, GltfPbrExtensionData, GltfPbrExtrasPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GltfPbrExtrasPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, add_iridescent_materials)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(2.0, 1.0, 8.0).looking_at(vec3(0.0, -0.2, 0.0), Vec3::Y),
        camera: Camera {
            hdr: true,
            ..default()
        },
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });

    commands.spawn(SceneBundle {
        scene: asset_server.load(GltfAssetLabel::Scene(0).from_asset("IridescenceSuzanne.gltf")),
        ..default()
    });
}

fn add_iridescent_materials(
    mut commands: Commands,
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
                // TODO: texture
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
                    iridescence_thickness_minimum: 200.0,
                    iridescence_thickness_maximum: 800.0,
                },
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
