use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
pub struct Health {
    pub hp: f32,
    pub last_hp: f32,
}
