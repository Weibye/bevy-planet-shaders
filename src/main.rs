use std::f32::consts::PI;

use bevy::{
    asset::LoadState, core_pipeline::Skybox, pbr::{CascadeShadowConfigBuilder, ExtendedMaterial}, prelude::*, render::render_resource::{Extent3d, TextureViewDescriptor, TextureViewDimension}
};
use celestial_shaders::{AtmosphereMaterial, CelestialShadersPlugin, PlanetMaterial};
use rand::Rng;

mod celestial_data;
mod celestial_shaders;
mod skybox;

use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_shader_utils::ShaderUtilsPlugin;



fn main() {
    App::new()
        .add_plugins((DefaultPlugins, 
            (
                CelestialShadersPlugin,
                ShaderUtilsPlugin,
                PanOrbitCameraPlugin
            )))
        .add_systems(Startup, setup)
        .add_systems(Update, (orbit_sun, create_new_seed))
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut planet_mats: ResMut<Assets<ExtendedMaterial<StandardMaterial, PlanetMaterial>>>,
    mut atmo_mats: ResMut<Assets<ExtendedMaterial<StandardMaterial, AtmosphereMaterial>>>,
    asset_server: Res<AssetServer>,
) {
    let mut rng = rand::thread_rng();
    
    // Create planet
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Sphere {
            radius: 180.0,
        }.mesh()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        material: planet_mats.add(ExtendedMaterial {
            base: StandardMaterial {
                base_color: Color::rgb(0.0, 0.0, 1.0),
                ..Default::default()
            },
            extension: PlanetMaterial {
                // planet_radius: 180.0,
                planet_seed: rng.gen(),
            },
        }),
        ..default()
    });
    // // Create atmosphere
    // commands.spawn(MaterialMeshBundle {
    //     mesh: meshes.add(Sphere {
    //         radius: 200.0,
    //     }.mesh()),
    //     transform: Transform::from_xyz(0.0, 0.0, 0.0),
    //     material: atmo_mats.add(ExtendedMaterial {
    //         base: StandardMaterial {
    //             base_color: Color::rgb(1.0, 1.0, 1.0),
    //             alpha_mode: AlphaMode::Blend,
    //             ..Default::default()
    //         },
    //         extension: AtmosphereMaterial {
    //             // planet_radius: 180.0,
    //             planet_radius: 180.0,
    //             atmosphere_radius: 200.0,
    //             atmosphere_color: Color::rgba(0.0, 0.0, 1.0, 0.1),
    //             atmosphere_density: 0.1,
    //         },
    //     }),
    //     ..default()
    // });

    let skybox_texture = skybox::generate_skybox(256, 256);
    let texture_handle = asset_server.add(skybox_texture);

    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 500.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PanOrbitCamera::default(),
        Skybox {
            image: texture_handle.clone(),
            brightness: 1000.0,
        },
    ));

    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 500.0
    });

    // directional 'sun' light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 400.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        // The default cascade config is designed to handle large scenes.
        // As this example has a much smaller world, we can tighten the shadow
        // bounds for better visual quality.
        cascade_shadow_config: CascadeShadowConfigBuilder {
            first_cascade_far_bound: 4.0,
            maximum_distance: 10.0,
            ..default()
        }
        .into(),
        ..default()
    });

    // commands.insert_resource(Cubemap {
    //     is_loaded: false,
    //     image_handle: texture_handle,
    // });
}

#[derive(Resource)]
struct Cubemap {
    is_loaded: bool,
    image_handle: Handle<Image>,
}

// fn asset_loaded(
//     asset_server: Res<AssetServer>,
//     mut images: ResMut<Assets<Image>>,
//     mut cubemap: ResMut<Cubemap>,
//     mut skyboxes: Query<&mut Skybox>,
// ) {
//     if !cubemap.is_loaded && asset_server.load_state(&cubemap.image_handle) == LoadState::Loaded {
        
//         let image = images.get_mut(&cubemap.image_handle).unwrap();
//         // NOTE: PNGs do not have any metadata that could indicate they contain a cubemap texture,
//         // so they appear as one texture. The following code reconfigures the texture as necessary.
//         if image.texture_descriptor.array_layer_count() == 1 {
//             image.reinterpret_stacked_2d_as_array(image.height() / image.width());
//             image.texture_view_descriptor = Some(TextureViewDescriptor {
//                 dimension: Some(TextureViewDimension::Cube),
//                 ..default()
//             });
//         }

//         for mut skybox in &mut skyboxes {
//             skybox.image = cubemap.image_handle.clone();
//         }

//         cubemap.is_loaded = true;
//     }
// }


fn orbit_sun(time: Res<Time>, mut sun_query: Query<(&mut Transform, &DirectionalLight)>) {
    for (mut transform, _) in sun_query.iter_mut() {
        transform.rotation = Quat::from_rotation_y(time.elapsed_seconds() as f32 * 0.1);
    }
}

fn create_new_seed(
    keys: Res<ButtonInput<KeyCode>>,
    mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, PlanetMaterial>>>,
) {
    // When the user presses space, we want to create a new seed
    if keys.just_pressed(KeyCode::Space) {
        let seed = rand::thread_rng().gen();
        materials.iter_mut().for_each(|(_handle, material)| {
            material.extension.planet_seed = seed;
        });
        println!("New Seed: {}", seed);
    }
    // query the planet material, then set a new seed
}
