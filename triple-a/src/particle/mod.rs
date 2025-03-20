pub mod gol_learn;
pub mod particle_data;
// pub mod particle_draw;

use std::ops::Deref;

use bevy::prelude::*;
use bevy::{
    asset::load_internal_asset,
    render::{
        Render, RenderApp, RenderSet,
        graph::CameraDriverLabel,
        render_graph::{RenderGraph, RenderLabel},
        render_resource::{binding_types::storage_buffer, *},
        renderer::RenderDevice,
    },
};
use particle_data::Particle;
// Tutorial:
// https://bayou-brogrammer.github.io/bevy_shader_playground/bevy_gol_example/part_1.html
//
// Remark:
//
// We have 2 World
// + used for normal bevy world
// + used for render world
//
// for add thing from bevy's world to render's world
// we need extract in ExtractSchedule
// https://docs.rs/bevy/latest/bevy/prelude/struct.ExtractSchedule.html

const PARTICLE_TYPES_SHADER: Handle<Shader> =
    Handle::weak_from_u128(232341360120154546900452057389924445265);
pub struct ParticlePlugin;
impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            PARTICLE_TYPES_SHADER,
            "../../assets/shaders/particle/types.wgsl",
            Shader::from_wgsl
        );
        let render_app = app.sub_app_mut(RenderApp);
        render_app
            .add_systems(ExtractSchedule, extract_effect_events)
            .add_systems(
                Render,
                prepare_bind_groups.in_set(RenderSet::PrepareBindGroups),
            );

        let mut graph = render_app.world_mut().resource_mut::<RenderGraph>();
        graph.add_node(ParticleLabel, ParticleNode);
        graph.add_node_edge(ParticleLabel, CameraDriverLabel);
    }

    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        render_app.insert_resource(GpuBuffers::new());
        render_app.init_resource::<ParticlePipelines>();
    }
}

/// used for extract resource
/// from Main World to Render World
///
/// In this function
/// we need collect all resource from MainWorld
/// put it into RenderWorld
fn extract_effect_events(
    // mut commands: Commands,
    device: Res<RenderDevice>,
    mut buffers: ResMut<GpuBuffers>,
) {
    // info!("extract_effect_events");

    // for demo 10 particle
    if buffers.particles.len() == 0 {
        info!("Added particle 10");
        buffers.particles.add();
        buffers.particles.add();
        buffers.particles.add();
        buffers.particles.add();
        buffers.particles.add();
        buffers.particles.add();
        buffers.particles.add();
        buffers.particles.add();
        buffers.particles.add();
        buffers.particles.add();
        buffers.particles.write_buffer(&device);
    } else {
        let k = buffers
            .particles
            .buffer()
            .unwrap()
            .slice(..)
            .get_mapped_range();
        // std::mem::size_of::<Particle>
        let k = k.to_vec();
        info!("{k:#?}");
    }
}

/// Pipline used for how phase function pixel pos will go
///
/// BindGroupLayout used for how we pass data layout into wgpu
#[derive(Resource)]
struct ParticlePipelines {
    /// present for function will be called
    init_pipeline: CachedComputePipelineId,
    /// present for function will be called
    // update_pipeline: CachedComputePipelineId,
    /// present for layout data passing into shader
    bind_group_layout: BindGroupLayout,
}

impl FromWorld for ParticlePipelines {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let bind_group_layout = render_device.create_bind_group_layout(
            "ParticlesLayout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::COMPUTE,
                (storage_buffer::<Vec<Particle>>(false),),
            ),
        );

        // load shader file
        let shader = world.load_asset::<Shader>("shaders/particle/main.wgsl");

        let pipeline_cache = world.resource::<PipelineCache>();

        let new_compute_pipeline = |entry_label: &'static str, shader: &Handle<Shader>| {
            pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
                label: Some(entry_label.into()),
                layout: vec![bind_group_layout.clone()],
                push_constant_ranges: vec![],
                shader: shader.clone(),
                shader_defs: vec![],
                entry_point: entry_label.into(),
                zero_initialize_workgroup_memory: false,
            })
        };

        let init_pipeline = new_compute_pipeline("init", &shader);
        // let update_pipeline = new_compute_pipeline("update", &shader);

        Self {
            init_pipeline,
            // update_pipeline,
            bind_group_layout,
        }
    }
}

/// Hold bind data to BindGroupLayout
#[derive(Resource)]
pub struct GpuBuffers {
    pub particles: UninitBufferVec<Particle>,
}

impl GpuBuffers {
    fn new() -> Self {
        Self {
            particles: UninitBufferVec::new(
                BufferUsages::STORAGE | BufferUsages::COPY_SRC | BufferUsages::COPY_DST,
            ),
        }
    }
}

#[derive(Resource, Deref)]
pub struct ParticleBindGroups(pub [BindGroup; 1]);
/// Bind all thing together
///
/// This region is Render' World
///
///
/// After extract_effect_events
/// we binding buffer into BindGroupLayout
///
/// so shader can access and modify it
fn prepare_bind_groups(
    mut commands: Commands,
    pipeline: Res<ParticlePipelines>,
    buffers: Res<GpuBuffers>,
    // gpu_images: Res<RenderAssets<GpuImage>>,
    render_device: Res<RenderDevice>,
) {
    // info!("prepare_bind_groups");
    let bind_group = render_device.create_bind_group(
        Some("ParticleBindGroup"),
        &pipeline.bind_group_layout,
        &BindGroupEntries::sequential((
            buffers.particles.binding().unwrap(),
            // buffers.settings.binding().unwrap(),
            // view_uniforms.uniforms.binding().unwrap(),
            // buffers.sorted_indices.binding().unwrap(),
            // buffers.counter.binding().unwrap(),
            // buffers.prefix_sum_reduction.binding().unwrap(),
            // buffers.prefix_sum_index.binding().unwrap(),
        )),
    );

    commands.insert_resource(ParticleBindGroups([bind_group]));
}

#[derive(RenderLabel, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ParticleLabel;

#[derive(Default)]
struct ParticleNode;

const WORKGROUP_SIZE: u32 = 64;

// TODO automatic this number
const NUM_PARTICLE: u32 = 64;

impl bevy::render::render_graph::Node for ParticleNode {
    fn run<'w>(
        &self,
        _graph: &mut bevy::render::render_graph::RenderGraphContext,
        render_context: &mut bevy::render::renderer::RenderContext<'w>,
        world: &'w World,
    ) -> Result<(), bevy::render::render_graph::NodeRunError> {
        let pipeline_cache = world.resource::<PipelineCache>();
        let bind_groups = world.resource::<ParticleBindGroups>();
        let pipeline = world.resource::<ParticlePipelines>();
        // let buffers = world.resource::<GpuBuffers>();

        let mut pass = render_context
            .command_encoder()
            .begin_compute_pass(&ComputePassDescriptor::default());

        macro_rules! get_pipeline {
            ($name:ident) => {
                match pipeline_cache.get_compute_pipeline(pipeline.$name) {
                    Some(pipeline) => pipeline,
                    None => return Ok(()),
                }
            };
            () => {};
        }

        // #Remark:
        // if we have 64 particle
        // we will dispatch 1 WORKGROUP
        //
        // config inside shader will have WORKGROUP_SIZE = 64
        // => we use 1 block
        // each block will have index from 0..64
        // cover all our index particle
        // each particle will have progress
        let workgroup_count = (NUM_PARTICLE as f32 / WORKGROUP_SIZE as f32).ceil() as u32;

        let init_pipeline = get_pipeline!(init_pipeline);
        // let update_pipeline = get_pipeline!(update_pipeline);

        pass.set_bind_group(0, &bind_groups[0], &[]);

        pass.set_pipeline(&init_pipeline);
        pass.dispatch_workgroups(workgroup_count, 1, 1);

        // pass.set_pipeline(&update_pipeline);
        // pass.dispatch_workgroups(workgroup_count, 1, 1);

        Ok(())
    }
}
