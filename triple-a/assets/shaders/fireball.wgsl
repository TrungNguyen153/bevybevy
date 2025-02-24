// https://github.com/bevyengine/bevy/blob/73d7e89a18e4dd0f0d2ad2294028e6b84a611088/crates/bevy_sprite/src/mesh2d/mesh2d_vertex_output.wgsl
// https://github.com/bevyengine/bevy/blob/73d7e89a18e4dd0f0d2ad2294028e6b84a611088/crates/bevy_render/src/globals.wgsl

#import bevy_sprite::mesh2d_view_bindings::{view, globals}
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var<uniform> canvas_width: f32;
@group(2) @binding(1) var<uniform> canvas_height: f32;

fn random2(st: vec2<f32>, seed: f32) -> vec2<f32> {
    let st2 = vec2<f32>( dot(st,vec2<f32>(127.1,311.7)),
              dot(st,vec2<f32>(269.5,183.3)) );
    return -1.0 + 2.0*fract(sin(st2)*43758.5453123 * seed * 0.753421);
}

// Remap value
fn map(value: f32 ,fromMin: f32 ,fromMax: f32 ,toMin: f32 ,toMax:  f32 ) -> f32
{
    var value2 = (value - fromMin) / (fromMax - fromMin);
    value2 = toMin + value2 * (toMax - toMin);
    return value2;
}

// Remap value with stepped lerp
fn mapStep(value: f32, fromMin:  f32, fromMax: f32, toMin: f32, toMax: f32, steps: f32 ) -> f32
{
    var value2 = (value - fromMin) / (fromMax - fromMin);
    value2 = floor(value2 * steps) / steps;
    value2 = toMin + value2 * (toMax - toMin);
    return value2;
}

// Blender's mixRGB node, linear light mode
fn linearLight(a: vec3<f32>, b: vec3<f32>, factor: f32 ) -> vec3<f32>
{
    return a + factor * (2. * b - 1.);
}

fn noise(st: vec2<f32>,seed: f32 ) -> f32 {
    let i = floor(st);
    let f = fract(st);

    let u = f*f*(3.0-2.0*f);

    return mix( mix( dot( random2(i + vec2<f32>(0.0,0.0), seed ), f - vec2<f32>(0.0,0.0) ),
                     dot( random2(i + vec2<f32>(1.0,0.0), seed ), f - vec2<f32>(1.0,0.0) ), u.x),
                mix( dot( random2(i + vec2<f32>(0.0,1.0), seed ), f - vec2<f32>(0.0,1.0) ),
                     dot( random2(i + vec2<f32>(1.0,1.0), seed ), f - vec2<f32>(1.0,1.0) ), u.x), u.y);
}

fn fbm (seed: f32 ,st: vec2<f32>,scale: f32 ,octaves: i32 ,roughness: f32,lacunarity: f32) -> f32
{
    // Initial values
    var amplitude = .5;
    let frequency = 0.;
    var value = 0.;
    var st2 = st * scale;
    //
    // Loop of octaves
    for (var i = 1; i < octaves; i++) {
        value += amplitude * noise(st2, seed);
        st2 *= lacunarity;
        amplitude *= roughness;
    }
    return value * .5 + .5;
}

/// Clockwise by `theta`
fn rotate2D(theta: f32) -> mat2x2<f32> {
    let c = cos(theta);
    let s = sin(theta);
    return mat2x2<f32>(c, s, -s, c);
}

fn sdfCircle(p: vec2<f32>, r: f32) -> f32 {
    return length(p) - r;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // vec4 shader:
    // xyzw
    // rgba
    //
    let fragCoord = in.position.xy;
    let iResolution = vec2<f32>(canvas_width, canvas_height);
    let iTime = globals.time;

    // calculate center
    var uv: vec2<f32> = in.uv;
    let uvX: f32 = uv.x;

    uv = vec2<f32>(uv.x, 1 - uv.y);
    
    uv = uv * 2.0 - 1.0;
    
    // uv *= rotate2D(3.14159/2.);
    uv.x *= iResolution.x / iResolution.y;

    var uvFlame = uv + vec2<f32>(iTime * 2., 0.);

    let roughness = 0.675;
    let detail = 4;
    let scale = 4.0;
    let lacunarity = 2.0;

    var noise1d: f32 = fbm(24., uvFlame, scale, detail, roughness, lacunarity);
    var noise3d: vec3<f32> = vec3<f32>(fbm(24., uvFlame, scale, detail, roughness, lacunarity), 
                        fbm(12., uvFlame, scale, detail, roughness, lacunarity),
                        fbm(33., uvFlame, scale, detail, roughness, lacunarity));

    var lightFactor : f32 = clamp(map(uvX, 0.13, .87, 1., .06), .06, 1.);

    var light: vec3<f32> = linearLight(vec3(uv * vec2(1., 1.), 0.), noise3d, lightFactor);
    light = abs(light) - vec3<f32>(.75, 0., 0.);
    light = max(light, vec3<f32>(0.));

    var lightLength: f32 = length(light);
    var fireball_grad : f32 = clamp(map(uvX, -.24, 0.82, 0.0, 0.27), 0.0, 0.27);
    lightLength -= fireball_grad;
    lightLength = step(lightLength, -.01);

    noise1d *= uvX;
    noise1d = mapStep(noise1d, .24, .77, .0, 2., 4.);
    noise1d *= pow(uvX, 4.);

    let color : vec3<f32> = mix(vec3<f32>(.33), vec3(1., .33, .068) * noise1d * 4., lightLength);

    return vec4<f32>(color, 1.0);
}
