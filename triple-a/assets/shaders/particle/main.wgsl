#import types::particles

const WORKGROUP_SIZE: u32 = 64;

@compute @workgroup_size(WORKGROUP_SIZE)
fn init(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
    particles[invocation_id.x].color += u32(1);
}

// @compute @workgroup_size(WORKGROUP_SIZE)
// fn update(@builtin(global_invocation_id) invocation_id: vec3<u32>) {
// }
