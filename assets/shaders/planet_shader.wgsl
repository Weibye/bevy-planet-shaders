// // #import bevy_pbr::{
// //     pbr_functions::alpha_discard,
// // }

// // #ifdef PREPASS_PIPELINE
// // #import bevy_pbr::{
// //     prepass_io::{VertexOutput, FragmentOutput},
// //     pbr_deferred_functions::deferred_output,
// // }
// // #else
// // #import bevy_pbr::{
// //     pbr_fragment::pbr_input_from_standard_material,
// //     forward_io::{VertexOutput, FragmentOutput},
// //     pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
// // }
// // #endif

// // #import bevy_shader_utils::{
// //     simplex_noise_3d::simplex_noise_3d,
// //     perlin_noise_2d::perlin_noise_2d,
// //     perlin_noise_3d::perlin_noise_3d,
// //     voronoise::voronoise
// // }

// #import "shaders/noise.wgsl"::rand11;

// // Color Palette
// const color_black = vec3(0.0, 0.0, 0.0);
// const color_orange = vec3(1.0, 0.0, 1.0);
// // Water
// const color_water_deep_deep_ocean = vec3(0.16, 0.50, 0.61);
// const color_water_deep_ocean = vec3(0.235, 0.592, 0.666);
// const color_water_ocean = vec3(0.254, 0.647, 0.705);
// const color_water_shallow = vec3(0.360, 0.682, 0.725);

// // Land
// const color_vegetation_lush = vec3(0.282, 0.690, 0.396);
// const color_vegetation = vec3(0.588, 0.784, 0.411);
// const color_vegetation_plains = vec3(0.886, 0.847, 0.568);
// const color_dirt = vec3(0.8, 0.721, 0.576);
// const color_highland = vec3(0.650, 0.568, 0.462);
// const color_mountain = vec3(0.486, 0.411, 0.352);
// const color_mountain_peaks = vec3(0.8, 0.8, 0.8);

// struct PlanetMaterial {
//     // planet_radius: f32,
//     planet_seed: u32,
// }

// @group(2) @binding(100)
// var<uniform> planet_material: PlanetMaterial;

// fn fmod(a: f32, b: f32) -> f32 {
//     return a - b * floor(a / b);
// }

// fn lerp(a: f32, b: f32, t: f32) -> f32 {
//     return a + t * (b - a);
// }

// fn norm(min: f32, max: f32, value: f32) -> f32 {
//     return (value - min) / (max - min);
// }

// fn offset(uv: vec2<f32>, offset: vec2<f32>) -> vec2<f32> {
//     return uv + offset;
// }

// fn swirl(uv: vec2<f32>, center: vec2<f32>, strength: f32) -> vec2<f32> {
//     let offset = uv - center;
//     let angle = strength * length(offset);
//     let swirled = vec2<f32>(
//         cos(angle) * offset.x - sin(angle) * offset.y,
//         sin(angle) * offset.x + cos(angle) * offset.y
//     );
//     return center + swirled;
// }



// @fragment
// fn fragment(
//     in: VertexOutput,
//     @builtin(front_facing) is_front: bool,
// ) -> FragmentOutput {

//     // generate a PbrInput struct from the StandardMaterial bindings
//     var pbr_input = pbr_input_from_standard_material(in, is_front);

//     let seed = f32(planet_material.planet_seed);
//     let random = rand11(seed);

//     // let oct_01_A: f32 = simplex_noise_3d(in.world_position.xyz * lerp(0.003, 0.01, rand11(seed + 1)) + lerp(1.0, 2.0, rand11(seed + 1)));
//     // let oct_01_B: f32 = simplex_noise_3d(in.world_position.xyz * lerp(0.003, 0.01, rand11(seed + 2)) + lerp(1.0, 2.0, rand11(seed + 2)));
//     // let oct_02_A: f32 = simplex_noise_3d(in.world_position.xyz * lerp(0.014, 0.025, rand11(seed + 3)) + lerp(0.5, 5.0, rand11(seed + 3)));
//     // let oct_02_B: f32 = simplex_noise_3d(in.world_position.xyz * lerp(0.014, 0.025, rand11(seed + 4)) + lerp(0.5, 5.0, rand11(seed + 4)));
//     // let oct_03_A: f32 = simplex_noise_3d(in.world_position.xyz * lerp(0.08, 0.10, rand11(seed + 5)) + lerp(0.5, 7.0, rand11(seed + 5)));
//     // let oct_03_B: f32 = simplex_noise_3d(in.world_position.xyz * lerp(0.08, 0.10, rand11(seed + 6)) + lerp(0.5, 7.0, rand11(seed + 6)));
//     // let oct_04_A: f32 = simplex_noise_3d(in.world_position.xyz * lerp(1.5, 0.3, rand11(seed + 7)) + lerp(0.5, 7.0, rand11(seed + 7)));
//     // let oct_04_B: f32 = simplex_noise_3d(in.world_position.xyz * lerp(1.5, 0.3, rand11(seed + 8)) + lerp(0.5, 7.0, rand11(seed + 8)));

    
//     // var voronoi_01 = voronoise(vec2(in.uv.x * lerp(5.0, 20.0, rand11(seed + 9)), in.uv.y * lerp(5.0, 20.0, rand11(seed + 10))), 1., 1.0);
//     // var voronoi_02 = voronoise(vec2(in.uv.x * lerp(1.0, 3.0, rand11(seed + 11)), in.uv.y * lerp(1.0, 3.0, rand11(seed + 12))), 1., 1.0);
//     // var voronoi_03 = voronoise(vec2(in.uv.x * lerp(0.1, 0.3, rand11(seed + 13)), in.uv.y * lerp(0.1, 0.3, rand11(seed + 14))), 1., 1.0);
//     // var voronoi_04 = voronoise(vec2(in.uv.x * lerp(0.01, 0.03, rand11(seed + 15)), in.uv.y * lerp(0.01, 0.3, rand11(seed + 16))), 1., 1.0);
    
//     // let oct_01 = oct_01_A + (oct_01_B * 0.5) * voronoi_01 + 0.4;
//     // let oct_02 = oct_02_A + (oct_02_B * 0.5) * voronoi_02 + 0.2;
//     // let oct_03 = oct_03_A + (oct_03_B * 0.5) * voronoi_03 + 0.1;
//     // let oct_04 = oct_04_A + (oct_04_B * 0.5) * voronoi_04;

//     // var elevation = clamp(0.0, 1.0, (oct_01 * 1.0 + oct_02 * 0.5 + oct_03 * 0.15 + oct_04 * 0.02) / 4.0);

//     // let elevation_map = vec3(elevation, elevation, elevation);

//     // let wind_bands = 6.0;
//     // let distance_from_poles = 1.0 - abs(in.uv.y - 0.5) * 2.0;
//     // let wind_pattern = abs(fmod(distance_from_poles * (wind_bands / 2.0), 2.0) - 1.0);
    
//     // let polar_region = smoothstep(0.45, 0.2, distance_from_poles);
//     // let equator_region = smoothstep(0.15, 0.0, 1.0 - distance_from_poles);

//     // let humidity_map = polar_region + equator_region;

//     // let poles = smoothstep(0.3, 0.4, distance_from_poles);

//     // let water_threshold = 0.15;
//     // let water_area_map = 1.0 - step(water_threshold, elevation);
//     // let water_normalized_elevation = norm(0.0, water_threshold, elevation * water_area_map);

//     // var water_topographic_map = vec3(water_normalized_elevation, water_normalized_elevation, water_normalized_elevation);
//     // water_topographic_map = mix(color_water_deep_deep_ocean, color_water_deep_ocean, step(0.3, water_normalized_elevation));
//     // water_topographic_map = mix(water_topographic_map, color_water_ocean, step(0.4, water_normalized_elevation));
//     // water_topographic_map = mix(water_topographic_map, color_water_shallow, step(0.8, water_normalized_elevation));
//     // water_topographic_map *= water_area_map; // Restrict water to water areas

//     // let land_area_map = 1.0 - water_area_map;

//     // // Mountain Ranges
//     // let perlin_a = perlin_noise_2d(in.uv * 10.0 + 5.0);
//     // let perlin_b = perlin_noise_2d(in.uv * 11.0);
//     // let perlin_c = perlin_noise_2d(in.uv * 50.0);
//     // let perlin_d = perlin_noise_2d(in.uv * 100.0 + 15.0);

//     // var final_noise = (abs((perlin_a + perlin_b * 0.1 + perlin_c * 0.2 + perlin_d * 0.1) / 4.0) * -1) + 0.1;
//     // var land_normalized_elevation = norm(water_threshold, 0.5, elevation * land_area_map);
//     // // land_normalized_elevation += perlin_a * 1.0 - perlin_b * 0.6 + perlin_c * 0.45;
//     // // land_normalized_elevation *= land_area_map;

//     // var land_topographic_map = vec3(land_normalized_elevation, land_normalized_elevation, land_normalized_elevation);

//     // land_topographic_map = mix(color_dirt, color_vegetation, step(0.15, land_normalized_elevation));
//     // land_topographic_map = mix(color_vegetation, color_vegetation_lush, step(0.15, land_normalized_elevation));
//     // land_topographic_map = mix(land_topographic_map, color_vegetation_plains, step(0.30, land_normalized_elevation));
//     // land_topographic_map = mix(land_topographic_map, color_dirt, step(0.65, land_normalized_elevation));
//     // land_topographic_map = mix(land_topographic_map, color_highland, step(0.68, land_normalized_elevation));
//     // land_topographic_map = mix(land_topographic_map, color_mountain, step(0.85, land_normalized_elevation));
//     // land_topographic_map = mix(land_topographic_map, color_mountain_peaks, step(0.91, land_normalized_elevation));

//     // let topographic_map = mix(water_topographic_map, land_topographic_map, land_area_map);

//     // // Mountain ranges
//     // // let scale_a = 20.0;
//     // // let scale_b = 40.0;
//     // // let a = voronoise(vec2(in.uv.x * 4.0, in.uv.y) * scale_a, 1.0, 0.6);
//     // // let b = voronoise(vec2(in.uv.x, in.uv.y * 2.0) * scale_b, 1.0, 0.4);
//     // // let result = (a * b); // - 1.0;
//     // // let res2 = fmod(result, 0.2);
//     // // // let b = voronoise(swirl(in.uv * 10.0, vec2(0.1, 0.5), 2.0), 1.0, 0.5);
//     // // // let c = voronoise(offset(in.uv, vec2(2.4, -3.3)) * 30.0, 1.0, 0.5);
//     // // var mountain_range = clamp(0.0, 1.0, (a + b) - 1.0);
//     // // let c = fmod(mountain_range, 0.1); // step(0.4, mountain_range);
//     // // let d = step(0.5, c);

//     // // mountain_range = res2;

//     // final_noise = land_normalized_elevation;

//     // let final_color = vec3(final_noise, final_noise, final_noise);

//     // // pbr_input.material.base_color = vec4(final_color, 1.0);
//     // pbr_input.material.base_color = vec4(topographic_map, 1.0);
//     // // pbr_input.material.reflectance = water_area_map;

//     // let overlay = mix(color_black, color_orange, wind_pattern);

//     pbr_input.color = vec4(color_orange, 1.0);
    
//     var out: FragmentOutput;
//     out.color = apply_pbr_lighting(pbr_input);
    
//     return out;
// }