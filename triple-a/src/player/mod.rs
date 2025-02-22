use bevy::prelude::*;

use crate::{
    GameAssets, GameState,
    components::{health::Health, sprite_animate::AnimationSpriteIndices},
    world_map::CHUNK_SIZE,
};
use avian2d::prelude::*;
pub const PLAYER_SPAWN_POS: Vec3 = Vec3::new(2.5 * CHUNK_SIZE, 16.0 + 2.5 * CHUNK_SIZE, 0.0);
pub const PLAYER_HEALTH: f32 = 10.0;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Player>()
            .add_systems(OnEnter(GameState::Gaming), spawn_player);
    }
}

#[derive(Component, Reflect, Default)]
#[require(Health, Sprite, Name, RigidBody(|| RigidBody::Dynamic))]
pub struct Player;

fn spawn_player(mut commands: Commands, assets: Res<GameAssets>) {
    println!("Spawn player");
    let entity = commands.spawn((
        Player,
        Sprite {
            image: assets.player_texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: assets.player_layout.clone(),
                ..Default::default()
            }),

            ..Default::default()
        },
        Transform::from_translation(PLAYER_SPAWN_POS).with_scale(Vec3::splat(2.0)),
        Health {
            hp: 100.,
            last_hp: 100.,
        },
        AnimationSpriteIndices::new_with_repeat_bundle(0, 5, 11),
    ));
}
