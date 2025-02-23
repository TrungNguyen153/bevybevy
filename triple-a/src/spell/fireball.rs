use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{GameAssets, components::movement::Facing};

use super::DamageType;

#[derive(Component, Reflect)]
#[require(Sprite)]
pub struct Fireball {
    pub timelife: Timer,
    pub base_damage: f32,
    pub damage_type: DamageType,
    pub critical_change: f32,
    pub piercing_count: usize,
}

pub fn spawn_fireball(
    command: &mut Commands,
    assets: &Res<GameAssets>,
    spawn_position: Vec3,
    facing: &Facing,
    critical_change: f32,
    projectiles: usize,
    piercing_count: usize,
) {
    for _ in 0..projectiles {
        command.spawn((
            Sprite {
                image: assets.fireball_texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: assets.fireball_layout.clone(),
                    ..Default::default()
                }),
                ..default()
            },
            RigidBody::Dynamic,
            Collider::circle(5.),
            Transform::from_translation(spawn_position),
            Sensor,
            Fireball {
                timelife: Timer::from_seconds(2., TimerMode::Once),
                base_damage: 10.,
                damage_type: DamageType::Fire,
                critical_change,
                piercing_count,
            },
            LinearVelocity(facing.truncate() * 40.),
        ));
    }
}

pub fn update_fireball(
    mut command: Commands,
    mut q_fireball: Query<(Entity, &mut Fireball), With<Fireball>>,
    time: Res<Time>,
) {
    let delta = time.delta();
    for (e, mut f) in &mut q_fireball {
        f.timelife.tick(delta);
        if f.timelife.just_finished() {
            command.entity(e).despawn_recursive();
        }
    }
}

// TODO
pub struct CustomFireballShader {}
