use bevy::prelude::*;

use crate::GameState;

use self::{
    health::Health,
    movement::{Facing, Velocity},
    sprite_animate::{
        AnimationSpriteIndices, AnimationSpriteRepeat, FrameTimer, execute_animate_sprites,
    },
};

pub mod health;
pub mod movement;
pub mod sprite_animate;

/// Commons components
pub struct ComponentPlugin;

impl Plugin for ComponentPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Health>()
            .register_type::<AnimationSpriteRepeat>()
            .register_type::<FrameTimer>()
            .register_type::<AnimationSpriteIndices>()
            .add_systems(
                Update,
                execute_animate_sprites.run_if(in_state(GameState::Gaming)),
            )
            .register_type::<Velocity>()
            .register_type::<Facing>()
            .add_systems(
                Update,
                (movement::update_position, movement::update_sprite_direction)
                    .run_if(in_state(GameState::Gaming)),
            );
    }
}
