#import bevy_pbr::{
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::{
        alpha_discard,
        apply_pbr_lighting,
        main_pass_post_lighting_processing
    },
    forward_io::{VertexOutput, FragmentOutput},
}

const PI = 3.14159265359;
const MAX = 10000.0;

const R_INNER = 1.0;
const R = R_INNER + 0.5;
const NUM_OUT_SCATTER = 8;
const NUM_IN_SCATTER = 80;

fn ray_sphere_intersection(p: vec3<f32>, dir: vec3<f32>, r: f32) -> vec2<f32> {
    let b = dot(p, dir);
    let c = dot(p, p) - r * r;

    var d = b * b - c;
    if d < 0.0 {
        return vec2(MAX, -MAX);
    }
    d = sqrt(d);

    return vec2(-b - d, -b + d);
}

// Mie Phase Function
fn phase_mie(g: f32, c: f32, cc: f32) -> f32 {
    let gg = g * g;

    let a = (1.0 - gg) * (1.0 + cc);

    var b = 1.0 + gg - 2.0 * g * c;
    b *= sqrt(b);
    b *= 2.0 + gg;

    return (3.0 / 8.0 / PI) * a / b;
}

// Rayleigh Phase Function
fn phase_ray(cc: f32) -> f32 {
    return (3.0 / 16.0 / PI) * (1.0 + cc);
}

fn ray_dir(fov: f32, size: vec2<f32>, pos: vec2<f32>) -> vec3<f32> {
    let xy = pos - size * 0.5;

    let cot_half_fov = tan(radians(90.0 - fov * 0.5));
    let z = size.y * 0.5 * cot_half_fov;

    return normalize(vec3<f32>(xy, -z));
}

fn density(p: vec3<f32>, ph: f32) -> f32 {
    return exp(-max(length(p) - R_INNER, 0.0) / ph);
}

// Calculates the optical depth along a path from point p to point q.
fn optic(p: vec3<f32>, q: vec3<f32>, ph: f32) -> f32 {
    let s: vec3<f32> = (q - p) / f32(NUM_OUT_SCATTER);
    var v: vec3<f32> = p + s * 0.5;
    
    var sum: f32 = 0.0;
    for (var i: i32 = 0; i < NUM_OUT_SCATTER; i = i + 1) {
        sum = sum + density(v, ph);
        v = v + s;
    }
    sum = sum * length(s);
    
    return sum;
}

fn in_scatter(o: vec3<f32>, dir: vec3<f32>, e: vec2<f32>, l: vec3<f32>) -> vec3<f32> {
    let ph_ray: f32 = 0.05;
    let ph_mie: f32 = 0.02;

    let k_ray: vec3<f32> = vec3<f32>(3.8, 13.5, 33.1);
    let k_mie: vec3<f32> = vec3<f32>(21.0, 21.0, 21.0); // WGSL does not support vec3<f32>(21.0) shorthand
    let k_mie_ex: f32 = 1.1;

    var sum_ray: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);
    var sum_mie: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);

    var n_ray0: f32 = 0.0;
    var n_mie0: f32 = 0.0;

    let len: f32 = (e.y - e.x) / f32(NUM_IN_SCATTER);
    let s: vec3<f32> = dir * len;
    var v: vec3<f32> = o + dir * (e.x + len * 0.5);

    for (var i: i32 = 0; i < NUM_IN_SCATTER; i = i + 1) {
        let d_ray: f32 = density(v, ph_ray) * len;
        let d_mie: f32 = density(v, ph_mie) * len;

        n_ray0 = n_ray0 + d_ray;
        n_mie0 = n_mie0 + d_mie;

        // The conditional preprocessor directive #if 0 is used to comment out code in GLSL.
        // In WGSL, you would simply not include the code or comment it out.

        let f: vec2<f32> = ray_sphere_intersection(v, l, R);
        let u: vec3<f32> = v + l * f.y;

        let n_ray1: f32 = optic(v, u, ph_ray);
        let n_mie1: f32 = optic(v, u, ph_mie);

        let att: vec3<f32> = exp(-(n_ray0 + n_ray1) * k_ray - (n_mie0 + n_mie1) * k_mie * k_mie_ex);

        sum_ray = sum_ray + d_ray * att;
        sum_mie = sum_mie + d_mie * att;
    }

    let c: f32 = dot(dir, -l);
    let cc: f32 = c * c;
    let scatter: vec3<f32> =
        sum_ray * k_ray * phase_ray(cc) +
        sum_mie * k_mie * phase_mie(-0.78, c, cc);

    return 10.0 * scatter;
}


@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    var pbr_input = pbr_input_from_standard_material(in, is_front);

    // Atmosphere parameters
    let atmosphere_thickness = 0.05;
    let atmosphere_color = vec3(0.62, 0.77, 0.89);

    // Calculate view angle
    let view_angle = dot(normalize(pbr_input.V), normalize(pbr_input.N));

    // Calculate atmosphere fade
    let atmosphere_fade = smoothstep(-1.0, 1.0, view_angle);

    // Mix atmosphere with surface color
    let final_color = mix(vec3(1.0, 1.0, 1.0), atmosphere_color, atmosphere_fade * atmosphere_thickness);

    // Apply to fragment output
    pbr_input.material.base_color = vec4(final_color, 0.5);

    let view = pbr_input.V;
    let direction = pbr_input.N;
    let light_direction: vec3<f32> = vec3<f32>(0.0, 0.0, 1.0);


    var e: vec2<f32> = ray_sphere_intersection(view, direction, R);
    if (e.x > e.y) {
        pbr_input.material.base_color = vec4<f32>(0.0, 0.0, 0.0, 1.0);
        // return out;
    } else {

        let f: vec2<f32> = ray_sphere_intersection(view, direction, R_INNER);
        e.y = min(e.y, f.x);

        let i: vec3<f32> = in_scatter(view, direction, e, light_direction);
        pbr_input.material.base_color = vec4(pow(i, vec3(1.0 / 2.2)), 1.0);
    
    }
    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr_input);
    
    return out;
}


// // Written by GLtracy

// // angle : pitch, yaw
// mat3 rot3xy( vec2 angle ) {
// 	vec2 c = cos( angle );
// 	vec2 s = sin( angle );
	
// 	return mat3(
// 		c.y      ,  0.0, -s.y,
// 		s.y * s.x,  c.x,  c.y * s.x,
// 		s.y * c.x, -s.x,  c.y * c.x
// 	);
// }

// void mainImage( out vec4 fragColor, in vec2 fragCoord )
// {
// 	// default ray dir
// 	vec3 dir = ray_dir( 45.0, iResolution.xy, fragCoord.xy );
	
// 	// default ray origin
// 	vec3 eye = vec3( 0.0, 0.0, 3.0 );

// 	// rotate camera
// 	mat3 rot = rot3xy( vec2( 0.0, iTime * 0.5 ) );
// 	dir = rot * dir;
// 	eye = rot * eye;
	
// 	// sun light dir
// 	vec3 l = vec3( 0.0, 0.0, 1.0 );
			  
// 	vec2 e = ray_vs_sphere( eye, dir, R );
// 	if ( e.x > e.y ) {
// 		fragColor = vec4( 0.0, 0.0, 0.0, 1.0 );
//         return;
// 	}
	
// 	vec2 f = ray_vs_sphere( eye, dir, R_INNER );
// 	e.y = min( e.y, f.x );

// 	vec3 I = in_scatter( eye, dir, e, l );
	
// 	fragColor = vec4( pow( I, vec3( 1.0 / 2.2 ) ), 1.0 );
// }