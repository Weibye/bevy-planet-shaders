#import bevy_pbr::forward_io::VertexOutput

#import bevy_shader_utils::{
    simplex_noise_3d::simplex_noise_3d,
    simplex_noise_2d::simplex_noise_2d,
}

@group(2) @binding(100) var<uniform> seed: u32;

@fragment
fn fragment(
    in: VertexOutput,
) -> @location(0) vec4<f32> {

    // Layered noise for star density and placement
    let base_noise = simplex_noise_2d(in.uv.xy * 1.0);
    return vec4<f32>(base_noise, base_noise, base_noise, 1.0);
    
    // let detail_noise = simplex_noise_2d(in.uv.xy * 50.0);
    // let quantized_noise = floor(detail_noise * 20.0) / 20.0; // Quantize space

    // let star_presence = base_noise * quantized_noise; // Combine noises for star density and placement
    // let star_threshold = 0.6; // Adjust for overall star density

    // // Determine if the current fragment is a star based on the noise value
    // if (base_noise > star_threshold) {
    //     // Generate a color based on the position to vary star colors
    //     // This is a simple example; you can use more complex logic for color variation
    //     let color = vec3<f32>(
    //         fract(sin(dot(in.world_position.xyz, vec3<f32>(12.9898, 78.233, 54.53))) * 43758.5453),
    //         fract(sin(dot(in.world_position.xyz, vec3<f32>(4.898, 7.233, 9.53))) * 43758.5453),
    //         fract(sin(dot(in.world_position.xyz, vec3<f32>(2.989, 3.233, 6.53))) * 43758.5453)
    //     );

    //     // Vary the brightness of the star to simulate different sizes
    //     let size_factor = fract(sin(dot(in.world_position.xyz, vec3<f32>(1.989, 2.233, 3.53))) * 43758.5453);
    //     let brightness = mix(0.5, 1.0, size_factor); // Mix between min and max brightness based on size_factor

    //     return vec4<f32>(color * brightness, 1.0); // Star with varying color and size (brightness)

    // } else {
    //     // Sky background color
    //     let final_color = vec4<f32>(0.0, 0.0, 0.0, 1.0);
    //     return final_color;
    // }
}