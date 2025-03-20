#define_import_path types

@group(0) @binding(0) var<storage, read_write> particles: array<Particle>;

struct Particle {
    position: vec2f,
    velocity: vec2f,
    color: u32,
    padding: u32,
}
