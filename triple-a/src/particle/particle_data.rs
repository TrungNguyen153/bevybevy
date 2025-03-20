use bevy::{prelude::*, render::render_resource::*};

#[derive(Component, ShaderType, Default, Debug, Clone, Copy)]
pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub color: u32,
    pub padding: u32,
}
