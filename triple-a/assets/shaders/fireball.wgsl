
/// Clockwise by `theta`
fn rotate2D(theta: f32) -> mat2x2<f32> {
    let c = cos(theta);
    let s = sin(theta);
    return mat2x2<f32>(c, s, -s, c);
}

// https://iquilezles.org/articles/distfunctions2d/
// learn SDFs:
// https://www.shadertoy.com/view/XsyGRW
// Begin SDFs
fn sdCircle(p: vec2<f32>, r: f32) -> f32 {
    return length(p) - r;
}

// distance from P to line AB
fn sdSegment(p: vec2<f32>, a: vec2<f32>, b: vec2<f32>) -> f32 {
    let pa = p - a;
    let ba = b - a;
    let h = clamp(dot(pa, ba) / dot(ba, ba), 0.0, 1.0);
    return length(pa - ba * h);
}

// End SDFs

// Begin utils


// Blender's mixRGB node, linear light mode
fn linear_light(a: vec3<f32>, b: vec3<f32>, factor: f32) -> vec3<f32> {
    return a + factor * (2. * b - 1.);
}

fn lerp_step(value: f32, from_min: f32, from_max: f32, to_min: f32, to_max: f32, steps: f32) -> f32 {
    var result = value;
    result = (result - from_min) / (from_max - from_min);
    result = floor(result * steps) / steps;
    result = to_min + result * (to_max - to_min);
    return result;
}

// Hàm trượt value theo độ dốc
fn lerp(value: f32, from_min: f32, from_max: f32, to_min: f32, to_max: f32) -> f32 {
    var result = value;
    result = (result - from_min) / (from_max - from_min);
    result = to_min + result * (to_max - to_min);
    return result;
}

// End utils

// Begin noise

// from book of shader
fn random2d(st: vec2<f32>) -> f32 {
    return fract(sin(dot(st, vec2<f32>(12.9898, 78.233))) * 43758.761);
}

// Ken Perlin Noise
fn noise(st: vec2<f32>, seed: f32) -> f32 {
    // Lấy phần nguyên
    let i = floor(st);
    // Lấy phần dư khi chia cho 1.0
    let f = fract(st);

    // Four corners in 2D of a tile
    let a = random2d(i);
    let b = random2d(i + vec2(1.0, 0.0));
    let c = random2d(i + vec2(0.0, 1.0));
    let d = random2d(i + vec2(1.0, 1.0));

    // Smooth Interpolation

    // Cubic Hermine Curve.  Same as SmoothStep()
    let u: vec2<f32> = f * f * (3.0 - 2.0 * f);
    // u = smoothstep(0.,1.,f);
    
    // Mix 4 coorners percentages
    return mix(a, b, u.x) + (c - a) * u.y * (1.0 - u.x) + (d - b) * u.x * u.y;
}

// noise hợp âm quảng 8
fn fbm(st: vec2<f32>, octaves: u32, st_scale: f32, seed: f32) -> f32 {
    // Initial values
    var amplitude = .5;
    var frequency = 0.;
    var value = 0.;

    var st_mut = st * st_scale;

    // Loop of octaves
    for (var i: u32 = 0; i < octaves; i++) {
        value = amplitude * noise(st_mut, seed);
        st_mut *= 2.;
        amplitude *= .5;
    }

    return value;
}
// End noise

fn debugSTCircle(st: vec2<f32>) -> vec4<f32> {
    let d = sdCircle(st, 0.1);
    if d > 0 {
        return vec4<f32>(1., .33, .068, .8);
    }
    return vec4<f32>(0., 0., 0., .8);
}
// uv coordinates start from the upper left corner (v-axis is facing down).
// st coordinates start from the lower left corner (t-axis is facing up).
// s = u;
// t = 1-v;
// https://github.com/bevyengine/bevy/blob/73d7e89a18e4dd0f0d2ad2294028e6b84a611088/crates/bevy_sprite/src/mesh2d/mesh2d_vertex_output.wgsl
// https://github.com/bevyengine/bevy/blob/73d7e89a18e4dd0f0d2ad2294028e6b84a611088/crates/bevy_render/src/globals.wgsl
#import bevy_sprite::mesh2d_view_bindings::{view, globals}
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var<uniform> iResolution: vec2<f32>;
@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // vec4 shader:
    // xyzw
    // rgba
    let fragCoord = in.position.xy;
    let iTime = globals.time;
    // learn: surface coordinates (st)

    // calculate center
    var uv: vec2<f32> = in.uv;
    // hold old uv center
    let uvX: f32 = uv.x;

    uv = uv * 2.0 - 1.0;
    uv.x *= iResolution.x / iResolution.y;

    // Move camera to right if uv.x +
    let flame_uv = uv + vec2<f32>(iTime * 1., 0.);

    let FLAME_BASE_COLOR = vec3<f32>(1., .33, .068);

    let octaves: u32 = u32(4);
    var noise_1d: f32 = fbm(flame_uv, octaves, 4., 66.);
    let noise_3d: vec3<f32> = vec3<f32>(
        fbm(flame_uv, octaves, 4., 66.),
        fbm(flame_uv, octaves, 4., 44.),
        fbm(flame_uv, octaves, 4., 24.)
    );

    // color gen for X
    // clamp .06 -> 1.
    let light_factor = clamp(lerp(uvX, 0.13, .87, 1., .06), .06, 1.);
    var light: vec3<f32> = linear_light(vec3<f32>(uv * vec2(1., 1.), 0.), noise_3d, light_factor);
    light = abs(light) - vec3<f32>(.75, 0., 0.);
    light = max(light, vec3<f32>(0.));
    let light_length = length(light);

    // noise_1d *= uvX;
    // noise_1d = lerp_step(noise_1d, .24, .77, .0, 2., 4.);
    // noise_1d *= pow(uvX, 4.);

    let color: vec3<f32> = mix(vec3<f32>(.33), vec3<f32>(1., .33, .068) * noise_1d * 4., light_length);

    return vec4<f32>(color, 1.);
}
