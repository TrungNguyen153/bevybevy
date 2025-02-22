use bevy::prelude::*;

use self::{
    health::Health,
    sprite_animate::{
        AnimationSpriteIndices, AnimationSpriteRepeat, FrameTimer, execute_animate_sprites,
    },
};

pub mod health;
pub mod sprite_animate;

/// Commons components
pub struct ComponentPlugin;

impl Plugin for ComponentPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Health>()
            .register_type::<AnimationSpriteRepeat>()
            .register_type::<FrameTimer>()
            .register_type::<AnimationSpriteIndices>()
            .add_systems(Update, execute_animate_sprites);
    }
}
