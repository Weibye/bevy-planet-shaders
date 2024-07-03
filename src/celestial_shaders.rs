use bevy::{
    app::{App, Plugin}, 
    asset::Asset, 
    pbr::{
        ExtendedMaterial, Material, MaterialExtension, MaterialPlugin, StandardMaterial
    }, 
    reflect::TypePath, render::{
        color::Color, render_resource::{AsBindGroup, ShaderRef}
    } 
};

const PLANET_SHADER_ASSET_PATH: &str = "shaders/planet_shader.wgsl";
const ATMOSPHERE_SHADER_ASSET_PATH: &str = "shaders/atmosphere_shader.wgsl";
const SKYBOX_SHADER_ASSET_PATH: &str = "shaders/skybox.wgsl";

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct PlanetMaterial {
    // planet_radius: f32,
    
    // #[uniform(101)]
    #[uniform(100)]
    pub planet_seed: u32,
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
pub(crate) struct AtmosphereMaterial {
    pub(crate) planet_radius: f32,
    pub(crate) atmosphere_radius: f32,
    pub(crate) atmosphere_color: Color,
    pub(crate) atmosphere_density: f32,
}

impl MaterialExtension for AtmosphereMaterial {
    fn fragment_shader() -> ShaderRef {
        ATMOSPHERE_SHADER_ASSET_PATH.into()
    }

    fn deferred_fragment_shader() -> ShaderRef {
        ATMOSPHERE_SHADER_ASSET_PATH.into()
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub(crate) struct SkyboxMaterial {
    #[uniform(100)]
    pub seed: u32
}

impl Material for SkyboxMaterial {

    fn fragment_shader() -> ShaderRef {
        SKYBOX_SHADER_ASSET_PATH.into()
    }

    fn alpha_mode(&self) -> bevy::prelude::AlphaMode {
        bevy::prelude::AlphaMode::Opaque
    }
}


pub struct CelestialShadersPlugin;

impl Plugin for CelestialShadersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(( 
            MaterialPlugin::<ExtendedMaterial<StandardMaterial, PlanetMaterial>>::default(),
            MaterialPlugin::<ExtendedMaterial<StandardMaterial, AtmosphereMaterial>>::default(),
            MaterialPlugin::<SkyboxMaterial>::default(),
        ));
    }
}