#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput
#import bevy_render::globals::Globals

struct PostProcessSettings {
    color1: vec4<f32>,
    color2: vec4<f32>,
    resolution: vec2<f32>,
    driver: f32,
    movement_angle: f32,
#ifdef SIXTEEN_BYTE_ALIGNMENT
    // WebGL2 structs must be 16 byte aligned.
    _webgl2_padding: vec2<u32>
#endif
}

@group(0) @binding(0) var screen_texture: texture_2d<f32>;
@group(0) @binding(1) var texture_sampler: sampler;
@group(0) @binding(2) var<uniform> settings: PostProcessSettings;
@group(0) @binding(3) var<uniform> globals: Globals;

// Define constants
const DEG_TO_RAD: f32 = 0.017453292519943295; // π / 180
const TWO_PI: f32 = 6.283185307179586;        // 2π

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    // Driver value directly controls the growth, clamped between 0.0 and 1.0
    let progress = clamp(settings.driver, 0.0, 1.0);

    // Calculate direction vectors based on the movement angle
    let angle_rad = settings.movement_angle * DEG_TO_RAD;
    let direction = normalize(vec2<f32>(cos(angle_rad), sin(angle_rad)));

    // Center the UV coordinates
    let centered_uv = in.uv - vec2<f32>(0.5, 0.5);

    // Adjust UV coordinates based on the aspect ratio
    let aspect_ratio = settings.resolution.x / settings.resolution.y;
    let adjusted_centered_uv = centered_uv * vec2<f32>(aspect_ratio, 1.0);

    // Shift UV coordinates using the directional growth
    let shifted_centered_uv = adjusted_centered_uv + direction * progress;

    // Generate the grid of circular patterns
    let circle_columns = 16.0;
    let circle_rows = circle_columns * settings.resolution.y / settings.resolution.x;

    let s_factor = dot(shifted_centered_uv, direction) * TWO_PI * circle_columns;
    let t_factor = dot(shifted_centered_uv, vec2<f32>(-direction.y, direction.x)) * TWO_PI * circle_rows;

    // Compute the circular pattern
    let time_factor = globals.time * 0.5;
    let circle = -cos(s_factor + time_factor) * cos(t_factor + time_factor);

    // Adjust the threshold for growth based on progress
    let smooth_factor = 0.06;
    let threshold = 1.0 - progress * 2.5; // Higher progress makes the circles larger
    let lower_bound = threshold - smooth_factor;
    let upper_bound = threshold + smooth_factor;

    // Calculate the blending factor for color transition
    let mix_factor = smoothstep(lower_bound, upper_bound, circle);

    // Mix the colors based on the calculated factor
    let final_color = mix(settings.color1, settings.color2, mix_factor);

    return final_color;
}
