

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

// https://iquilezles.org/articles/distfunctions2d/
// Begin SDFs
fn sdCircle(p: vec2<f32>, r: f32) -> f32 {
    return length(p) - r;
}

// distance from P to line AB
fn sdSegment(p: vec2<f32>, a: vec2<f32>, b: vec2<f32>) -> f32
{
    let pa = p-a;
    let ba = b-a;
    let h = clamp( dot(pa,ba)/dot(ba,ba), 0.0, 1.0 );
    return length( pa - ba*h );
}

// End SDFs

fn drawLine(uv: vec2<f32>, p1: vec2<f32>, p2: vec2<f32>, thickness: f32) -> vec3<f32>
{
    let d = sdSegment(uv, p1, p2) - thickness;
    var col = vec3<f32>(1.0) - sign(d) * vec3<f32>(0.1, 0.4, 0.7);
	//col *= 1.0 - exp(-3.0*abs(d));
	//col *= 0.8 + 0.2*cos(120.0*d);
	col = mix( col, vec3(1.0), 1.0-smoothstep(0.0 ,0.015 ,abs(d)) );
    return col;
}

fn drawCircle(uv: vec2<f32>, radius: f32, colorInner: vec3<f32>, colorOuter: vec3<f32>) -> vec3<f32>
{
    let d = sdCircle(uv, radius);
    if (d > 0) {
        return colorOuter;
    }
    return colorInner;
}

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

    // calculate center
    var uv: vec2<f32> = in.uv;
    // hold old uv center
    let uvX: f32 = uv.x;
    uv = uv * 2.0 - 1.0;
    uv.x *= iResolution.x / iResolution.y;

    // learn:
    // https://www.shadertoy.com/view/XsyGRW

    let d = min(sdCircle(uv, 0.2), sdSegment(uv, vec2<f32>(0., 0.5), vec2<f32>(0., -.5)) - 0.01);
    if (d > 0) {
        return vec4<f32>(0., 0., 0., 1.);
    }
    // var col = drawLine(uv, vec2<f32>(0., 0.5), vec2<f32>(0., -.5), 0.);
    // col = drawCircle(uv,);

    return vec4<f32>(1.0);
}
