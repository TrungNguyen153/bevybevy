use bevy::prelude::*;

use self::health_changed::EventHealthChanged;
pub mod health_changed;

/// Commons events
pub struct EventPlugin;

impl Plugin for EventPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<EventHealthChanged>()
            .add_event::<EventHealthChanged>();
    }
}
