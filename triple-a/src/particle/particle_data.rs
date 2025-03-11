use bevy::{prelude::*, render::render_resource::*};

#[derive(Component, ShaderType, Default, Debug, Clone, Copy)]
pub struct Particle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub color: ColorId,
    pub padding: u32,
}

#[derive(ShaderType, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ColorId {
    pub id: u32,
}
