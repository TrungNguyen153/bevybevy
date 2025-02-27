// https://github.com/bevyengine/bevy/blob/73d7e89a18e4dd0f0d2ad2294028e6b84a611088/crates/bevy_sprite/src/mesh2d/mesh2d_vertex_output.wgsl
// https://github.com/bevyengine/bevy/blob/73d7e89a18e4dd0f0d2ad2294028e6b84a611088/crates/bevy_render/src/globals.wgsl
#import bevy_sprite::mesh2d_view_bindings::{view, globals}
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var<uniform> iResolution: vec2<f32>;
@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let iTime = globals.time;

    var uv: vec2<f32> = in.uv;
    let uvY = uv.y;
    uv.x *= iResolution.x / iResolution.y;
    uv = uv * 2. - 1.;
    var p = uv;
    let roughness = 0.675;
    let detail = 4;
    let scale = 4.0;
    let lacunarity = 2.0;
    var noise_1d = fbm(24., p + vec2(0., 2. * iTime), scale, detail, roughness, lacunarity);
    let noise_3d = vec3(fbm(24., p + vec2(0., iTime), scale, detail, roughness, lacunarity),
        fbm(12., p + vec2(0., iTime), scale, detail, roughness, lacunarity),
        fbm(33., p + vec2(0., iTime), scale, detail, roughness, lacunarity));

    let yGradient = clamp(0.35 - uv.y, 0., 1.) * 0.6;
    var circle_noise = vec2(noise_1d * 0.8, noise_1d * 2.5 * yGradient);
    p += circle_noise;
    let d = sdfCircle(p - vec2(1.2, 0.6), 0.4);
    let alpha = step(0., d);
    var col = vec3(alpha);

    var lightFactor = clamp(map(uvY, 0.1, .82, 1., .00), .00, 1.);
    let light = linearLight(vec3(uv, 0.), noise_3d, lightFactor);
    lightFactor = light.y;
    col = mix(col, vec3(1., .33, .068) * noise_1d * 3., lightFactor);

    return vec4f(col, 1. - alpha);
}

fn sdfCircle(p: vec2<f32>, r: f32) -> f32 {
    return length(p) - r;
}
 
fn random2(st: vec2<f32>, seed: f32) -> vec2<f32> {
    var st_var = st;
    st_var = vec2<f32>(dot(st_var, vec2<f32>(127.1, 311.7)), dot(st_var, vec2<f32>(269.5, 183.3)));
    return -1. + 2. * fract(sin(st_var) * 43758.547 * seed * 0.753421);
}
 
fn map(value: f32, fromMin: f32, fromMax: f32, toMin: f32, toMax: f32) -> f32 {
    var value_var = value;
    value_var = (value_var - fromMin) / (fromMax - fromMin);
    value_var = toMin + value_var * (toMax - toMin);
    return value_var;
}
 
fn mapStep(value: f32, fromMin: f32, fromMax: f32, toMin: f32, toMax: f32, steps: f32) -> f32 {
    var value_var = value;
    value_var = (value_var - fromMin) / (fromMax - fromMin);
    value_var = floor(value_var * steps) / steps;
    value_var = toMin + value_var * (toMax - toMin);
    return value_var;
}
 
fn linearLight(a: vec3<f32>, b: vec3<f32>, factor: f32) -> vec3<f32> {
    return a + factor * (2. * b - 1.);
}
 
fn noise(st: vec2<f32>, seed: f32) -> f32 {
    var i: vec2<f32> = floor(st);
    let f: vec2<f32> = fract(st);
    let u: vec2<f32> = f * f * (3. - 2. * f);
    return mix(mix(dot(random2(i + vec2<f32>(0., 0.), seed), f - vec2<f32>(0., 0.)), dot(random2(i + vec2<f32>(1., 0.), seed), f - vec2<f32>(1., 0.)), u.x), mix(dot(random2(i + vec2<f32>(0., 1.), seed), f - vec2<f32>(0., 1.)), dot(random2(i + vec2<f32>(1., 1.), seed), f - vec2<f32>(1., 1.)), u.x), u.y);
}
 
fn fbm(seed: f32, st: vec2<f32>, scale: f32, octaves: i32, roughness: f32, lacunarity: f32) -> f32 {
    var st_var = st;
    var amplitude: f32 = 0.5;
    let frequency: f32 = 0.;
    var value: f32 = 0.;
    st_var = st_var * (scale);

    for (var i: i32 = 1; i < octaves; i = i + 1) {
        value = value + (amplitude * noise(st_var, seed));
        st_var = st_var * (lacunarity);
        amplitude = amplitude * (roughness);
    }

    return value * 0.5 + 0.5;
}
