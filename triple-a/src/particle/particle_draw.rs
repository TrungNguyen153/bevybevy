use bevy::prelude::*;
use bevy::render::{
    RenderApp,
    graph::CameraDriverLabel,
    render_graph::RenderLabel,
    render_resource::{
        CachedRenderPipelineId, FragmentState, MultisampleState, PipelineCache, PrimitiveState,
        RenderPipelineDescriptor, VertexState,
    },
};

use super::ParticlePipelines;

pub struct ParticleDrawPlugin;
impl Plugin for ParticleDrawPlugin {
    fn build(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);

        let mut graph = render_app.world_mut().resource_mut::<RenderGraph>();
        graph.add_node(ParticleLabel, ParticleNode);
        graph.add_node_edge(ParticleLabel, CameraDriverLabel);
    }

    fn finish(&self, _app: &mut App) {
        //
    }
}

#[derive(Resource)]
struct ParticleDrawPipline {
    pipeline: CachedRenderPipelineId,
}

impl FromWorld for ParticleDrawPipline {
    fn from_world(world: &mut World) -> Self {
        let particle_pipline = world.resource::<ParticlePipelines>();

        let shader = world
            .resource::<AssetServer>()
            .load::<Shader>("shaders/particle/draw.wgsl");

        let pipeline_cache = world.resource::<PipelineCache>();

        let pipeline = pipeline_cache.queue_render_pipeline(RenderPipelineDescriptor {
            label: None,
            layout: vec![particle_pipline.bind_group_layout.clone()],
            push_constant_ranges: vec![],
            vertex: VertexState {
                shader: shader.clone(),
                shader_defs: vec![],
                entry_point: "vertex".into(),
                buffers: vec![],
            },
            primitive: PrimitiveState::default(),
            depth_stencil: None,
            multisample: MultisampleState {
                count: 4,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(FragmentState {
                shader,
                shader_defs: vec![],
                entry_point: "fragment".into(),
                targets: vec![],
            }),
            zero_initialize_workgroup_memory: true,
        });
        Self { pipeline }
    }
}

#[derive(RenderLabel, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DrawParticleLabel;

#[derive(Default)]
pub struct DrawParticleNode;
