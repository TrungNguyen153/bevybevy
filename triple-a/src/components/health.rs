use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
pub struct Health {
    pub max_hp: f32,
    pub hp: f32,
    pub last_hp: f32,
}

impl Health {
    pub fn percent(&self) -> f32 {
        self.hp / self.max_hp
    }

    pub fn last_changed(&self) -> f32 {
        self.last_hp - self.hp
    }
}
