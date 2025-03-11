const WORKGROUP_SIZE: u32 = 64;

@group(0) @binding(0) var<storage, read_write> particles: Particles;

struct Particles {
    particles: array<Particle>,
}

struct Particle {
    position: vec2<f32>,
    velocity: vec2<f32>,
    color: u32,
    padding: u32,
}

@compute @workgroup_size(WORKGROUP_SIZE)
fn init(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
}

@compute @workgroup_size(WORKGROUP_SIZE)
fn update(@builtin(global_invocation_id) invocation_id: vec3<u32>) {}
