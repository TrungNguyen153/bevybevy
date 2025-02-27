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
    // flip y for like shadertoy
    uv.y = 1. - uv.y;
    // uv.x *= iResolution.x / iResolution.y;

  
    // keep old uv x for moving
    var uvX: f32 = uv.x;
    var uvY: f32 = uv.x;
  
    // centering uv
    uv = uv * 2. -1;

    let uvFlame = uv + vec2(iTime * 2., 0.);

    let roughness = 0.675;
    let detail = 4;
    let scale = 4.0;
    let lacunarity = 2.0;

    var noise1d = fbm(24., uvFlame, scale, detail, roughness, lacunarity);
    var noise3d = vec3f(fbm(24., uvFlame, scale, detail, roughness, lacunarity),
        fbm(12., uvFlame, scale, detail, roughness, lacunarity),
        fbm(33., uvFlame, scale, detail, roughness, lacunarity));

    var lightFactor: f32 = clamp(map(uvX, 0.13, .95, 1., .06), .06, 1.);
    var light: vec3<f32> = linearLight(vec3(uv * vec2(1., 1.), 0.), noise3d, lightFactor);
    light = abs(light) - vec3f(.75, 0., 0.);
    light = max(light, vec3f(0.));
    var lightLength = length(light);

    let fireball_grad = clamp(map(uvX, -.24, 0.82, 0.0, 0.27), 0.0, 0.27);
    lightLength -= fireball_grad;
    lightLength = step(lightLength, -.01);


    noise1d *= uvX + 0.8;
    noise1d = mapStep(noise1d, .24, .77, .0, 2., 4.);
    noise1d *= pow(uvX, 4.);

    var col = mix(vec3(.33), vec3(1., .33, .068) * noise1d * 4., lightLength);
    // col = mix(vec4(col, 1.), vec4(col, 0.), 1.);

    return mix(vec4(col, 1.), vec4(col, 0.), .3);
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




