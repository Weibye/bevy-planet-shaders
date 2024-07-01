use bevy::{math::Vec3, prelude::default, render::{render_asset::RenderAssetUsages, render_resource::{Extent3d, TextureDimension, TextureFormat, TextureViewDescriptor, TextureViewDimension}, texture::Image}};
use noise::{NoiseFn, Perlin};

const SEED : u32 = 8564132;
const SCALE : f32 = 10.0;

pub(crate) fn generate_skybox(width: u32, height: u32) -> Image {

    println!("Generating skybox");
    let perlin = Perlin::new(SEED);

    let mut texture_data = Vec::new();

    // Generate texture data for each face of the cubemap
    for face in 0..6 {
        let face_color = match face {
            0 => (255, 0, 0), // r
            1 => (0, 255, 0), // g
            2 => (0, 0, 255), // b
            3 => (255, 255, 0), // y
            4 => (0, 255, 255), // c
            5 => (255, 0, 255), // m
            _ => (0, 0, 0),
        };
        for y in 0..height {
            for x in 0..width {
                let u = x as f32 / width as f32;
                let v = y as f32 / height as f32;
                let mut direction = face_uv_to_direction(face, u, v);
                direction *= SCALE;
                let noise_value = perlin.get([direction.x as f64, direction.y as f64, direction.z as f64]) as f32 + 0.5;
                let final_color = [(face_color.0 as f32 * noise_value) as u8, (face_color.1 as f32 * noise_value) as u8, (face_color.2 as f32 * noise_value) as u8, 255];
                texture_data.extend_from_slice(&final_color);
            }
        }
    }

    let mut result = Image::new_fill(
        Extent3d {
            width,
            height: height * 6,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &texture_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
    );


    result.reinterpret_stacked_2d_as_array(result.height() / result.width());
    result.texture_view_descriptor = Some(TextureViewDescriptor {
        dimension: Some(TextureViewDimension::Cube),
        ..default()
    });

    println!("Skybox generated");

    result

}

// Helper function to convert cubemap face pixel coordinates to a direction vector
fn face_uv_to_direction(face: usize, u: f32, v: f32) -> Vec3 {
    let x = (u - 0.5) * 2.0;
    let y = (v - 0.5) * 2.0;
    let z = 1.0;

    match face {
        0 => Vec3::new(z, y, x),   // Positive X
        1 => Vec3::new(-z, y, -x),   // Negative X
        2 => Vec3::new(x, -z, -y),   // Negative Y
        3 => Vec3::new(x, z, y),   // Positive Y
        4 => Vec3::new(x, y, -z),         // Positive Z
        5 => Vec3::new(-x, y, z),       // Negative Z
        _ => Vec3::new(0.0, 0.0, 0.0),
    }.normalize()
}