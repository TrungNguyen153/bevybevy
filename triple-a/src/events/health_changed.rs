use bevy::prelude::*;

#[derive(Event, Reflect)]
pub struct EventHealthChanged {
    pub entity: Entity,
    pub health_changed: f32,
}
