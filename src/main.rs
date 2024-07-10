use std::f32::consts::PI;

use bevy::{
    app::{App, Startup, Update},
    asset::{AssetServer, Assets, Handle},
    color::Color,
    input::ButtonInput,
    math::{Quat, Vec3},
    pbr::{
        light_consts, wireframe::Wireframe, AmbientLight, CascadeShadowConfigBuilder,
        DirectionalLight, DirectionalLightBundle, MaterialMeshBundle,
    },
    prelude::{Camera3dBundle, Commands, KeyCode, Query, Res, ResMut, Resource},
    render::{mesh::Mesh, texture::Image},
    time::Time,
    transform::components::Transform,
    utils::default,
    DefaultPlugins,
};
use celestial_shaders::{CelestialShadersPlugin, SkyboxMaterial};
use geometry::spherical_cuboid;
// use celestial_shaders::{
//     AtmosphereMaterial, CelestialShadersPlugin,
//     // PlanetMaterial,
//     SkyboxMaterial,
// };
use pcg_planet::PcgPlanetPlugin;
use rand::Rng;

mod celestial_data;
mod celestial_shaders;
mod geometry;
mod pcg_planet;
mod skybox;

use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
// use bevy_shader_utils::ShaderUtilsPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            CelestialShadersPlugin,
            // ShaderUtilsPlugin,
            PanOrbitCameraPlugin,
            PcgPlanetPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (orbit_sun, create_new_seed))
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    // mut planet_mats: ResMut<Assets<ExtendedMaterial<StandardMaterial, PlanetMaterial>>>,
    // mut atmo_mats: ResMut<Assets<ExtendedMaterial<StandardMaterial, AtmosphereMaterial>>>,
    mut skybox_mats: ResMut<Assets<SkyboxMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let mut rng = rand::thread_rng();
    const RADIUS: f32 = 180.0;

    // // Create planet
    // commands.spawn(MaterialMeshBundle {
    //     mesh: meshes.add(spherical_cuboid(RADIUS, 16, false, true)),
    //     transform: Transform::from_xyz(0.0, 0.0, 0.0),
    //     material: planet_mats.add(ExtendedMaterial {
    //         base: StandardMaterial {
    //             base_color: Color::srgb(0.0, 0.0, 1.0),
    //             ..Default::default()
    //         },
    //         extension: PlanetMaterial {
    //             // planet_radius: 180.0,
    //             planet_seed: rng.gen(),
    //         },
    //     }),
    //     ..default()
    // });
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

    // Skybox
    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(spherical_cuboid(35000.0, 32, true, true)),
            material: skybox_mats.add(SkyboxMaterial {
                seed: rng.gen::<u32>(),
            }),
            ..default()
        },
        Wireframe,
    ));

    // let skybox_texture = skybox::generate_skybox(256, 256);
    // let texture_handle = asset_server.add(skybox_texture);

    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 500.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PanOrbitCamera::default(),
        // Skybox {
        //     image: texture_handle.clone(),
        //     brightness: 1000.0,
        // },
    ));

    // ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 500.0,
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
}

#[derive(Resource)]
struct Cubemap {
    is_loaded: bool,
    image_handle: Handle<Image>,
}

fn orbit_sun(time: Res<Time>, mut sun_query: Query<(&mut Transform, &DirectionalLight)>) {
    for (mut transform, _) in sun_query.iter_mut() {
        transform.rotation = Quat::from_rotation_y(time.elapsed_seconds() as f32 * 0.1);
    }
}

fn create_new_seed(
    keys: Res<ButtonInput<KeyCode>>,
    // mut materials: ResMut<Assets<ExtendedMaterial<StandardMaterial, PlanetMaterial>>>,
) {
    // When the user presses space, we want to create a new seed
    if keys.just_pressed(KeyCode::Space) {
        let seed: u32 = rand::thread_rng().gen();
        // materials.iter_mut().for_each(|(_handle, material)| {
        //     material.extension.planet_seed = seed;
        // });
        println!("New Seed: {}", seed);
    }
    // query the planet material, then set a new seed
}
