use std::f32::consts::PI;

use bevy::{
    pbr::{CascadeShadowConfigBuilder, ExtendedMaterial, MaterialExtension},
    prelude::*,
    render::render_resource::*,
};
use rand::Rng;

use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_shader_utils::ShaderUtilsPlugin;

const PLANET_SHADER_ASSET_PATH: &str = "shaders/planet_shader.wgsl";
const ATMOSPHERE_SHADER_ASSET_PATH: &str = "shaders/atmosphere_shader.wgsl";

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, (
            MaterialPlugin::<ExtendedMaterial<StandardMaterial, PlanetMaterial>>::default()),
            MaterialPlugin::<ExtendedMaterial<StandardMaterial, AtmosphereMaterial>>::default(),
            ShaderUtilsPlugin,
            PanOrbitCameraPlugin
        ))
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
    // Create atmosphere
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Sphere {
            radius: 200.0,
        }.mesh()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        material: atmo_mats.add(ExtendedMaterial {
            base: StandardMaterial {
                base_color: Color::rgb(1.0, 1.0, 1.0),
                alpha_mode: AlphaMode::Blend,
                ..Default::default()
            },
            extension: AtmosphereMaterial {
                // planet_radius: 180.0,
                planet_radius: 180.0,
                atmosphere_radius: 200.0,
                atmosphere_color: Color::rgba(0.0, 0.0, 1.0, 0.1),
                atmosphere_density: 0.1,
            },
        }),
        ..default()
    });



    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-200.0, 250.0, 500.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PanOrbitCamera::default(),
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
}


#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct PlanetMaterial {
    // planet_radius: f32,
    
    // #[uniform(101)]
    #[uniform(100)]
    planet_seed: u32,
    // #[texture(1)]
    // #[sampler(2)]
    // color_texture: Option<Handle<Image>>,
    // alpha_mode: AlphaMode,
}

/// The Material trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material api docs for details!
impl MaterialExtension for PlanetMaterial {
    fn fragment_shader() -> ShaderRef {
        PLANET_SHADER_ASSET_PATH.into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        PLANET_SHADER_ASSET_PATH.into()
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct AtmosphereMaterial {
    planet_radius: f32,
    atmosphere_radius: f32,
    atmosphere_color: Color,
    atmosphere_density: f32,
}

impl MaterialExtension for AtmosphereMaterial {
    fn fragment_shader() -> ShaderRef {
        ATMOSPHERE_SHADER_ASSET_PATH.into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        ATMOSPHERE_SHADER_ASSET_PATH.into()
    }
}

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