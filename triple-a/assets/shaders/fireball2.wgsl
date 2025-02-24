
#import bevy_sprite::mesh2d_view_bindings::{view, globals}
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

@group(2) @binding(0) var<uniform> canvas_width: f32;
@group(2) @binding(1) var<uniform> canvas_height: f32;

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

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let R: vec2<f32> = vec2<f32>(canvas_width, canvas_height);
    
    let iTime = globals.time;
    let y_inverted_location = vec2<i32>(i32(in.position.x), i32(R.y) - i32(in.position.y));
    let location = vec2<i32>(i32(in.position.x), i32(in.position.y));
    
	var fragColor: vec4<f32>;
	var fragCoord = vec2<f32>(f32(location.x), f32(location.y) );

	var uv: vec2<f32> = fragCoord / R;
	let uvX: f32 = uv.x;
	uv = uv * 2. - 1.;
	let uvFlame: vec2<f32> = uv + vec2<f32>(iTime * 2., 0.);
	let roughness: f32 = 0.675;
	let detail: i32 = 4;
	let scale: f32 = 4.;
	let lacunarity: f32 = 2.;
	var noise1d: f32 = fbm(24., uvFlame, scale, detail, roughness, lacunarity);
	let noise3d: vec3<f32> = vec3<f32>(fbm(24., uvFlame, scale, detail, roughness, lacunarity), fbm(12., uvFlame, scale, detail, roughness, lacunarity), fbm(33., uvFlame, scale, detail, roughness, lacunarity));
	let lightFactor: f32 = clamp(map(uvX, 0.13, 0.87, 1., 0.06), 0.06, 1.);
	var light: vec3<f32> = linearLight(vec3<f32>(uv * vec2<f32>(1., 1.), 0.), noise3d, lightFactor);
	light = abs(light) - vec3<f32>(0.75, 0., 0.);
	light = max(light, vec3<f32>(0.));
	var lightLength: f32 = length(light);
	let fireball_grad: f32 = clamp(map(uvX, -0.24, 0.82, 0., 0.27), 0., 0.27);
	lightLength = lightLength - (fireball_grad);
	lightLength = step(lightLength, -0.01);
	noise1d = noise1d * (uvX);
	noise1d = mapStep(noise1d, 0.24, 0.77, 0., 2., 4.);
	noise1d = noise1d * (pow(uvX, 4.));
	let color: vec3<f32> = mix(vec3<f32>(0.33), vec3<f32>(1., 0.33, 0.068) * noise1d * 4., lightLength);
	return vec4<f32>(color, 1.);
} 

