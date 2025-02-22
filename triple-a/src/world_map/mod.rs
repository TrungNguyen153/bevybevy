use bevy::prelude::*;
pub mod camera;
pub mod map;

pub const BACKGROUND_ZINDEX_ABS: f32 = 1_000.0;
pub const CHUNK_SIZE: f32 = 32.0 * 32.0;

pub struct WorldMapPlugin;

impl Plugin for WorldMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((camera::CameraPlugin, map::MapPlugin));
    }
}
