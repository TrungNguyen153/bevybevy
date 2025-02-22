use std::time::SystemTime;

use avian2d::prelude::PhysicsSet;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_kira_audio::prelude::SpatialAudioReceiver;
use noisy_bevy::simplex_noise_2d_seeded;

use crate::player::Player;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MainCamera>()
            .init_resource::<CameraShake>()
            .add_systems(Startup, spawn_camera)
            .add_systems(
                PostUpdate,
                (
                    update_camera_target
                        .after(PhysicsSet::Sync)
                        .before(TransformSystem::TransformPropagate)
                        .before(update_camera),
                    update_camera.before(TransformSystem::TransformPropagate),
                ),
            );
    }
}

#[derive(Component, Reflect, Default)]
#[require(Name(|| "MainCamera"), SpatialAudioReceiver(|| SpatialAudioReceiver), Camera2d)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    println!("Spwan camera!!");
    commands.spawn((MainCamera, OrthographicProjection {
        scaling_mode: ScalingMode::FixedVertical {
            viewport_height: 400.0,
        },
        ..OrthographicProjection::default_2d()
    }));
}

const NOISE_STRENGTH: f32 = 10.0;
const TRANSLATION_SHAKE_STRENGTH: f32 = 15.0;
const ROTATION_SHAKE_STRENGTH: f32 = 2.5;

#[derive(Resource, Default, Reflect)]
pub struct CameraShake {
    trauma: f32,
    seed: f32,
    target: Vec2,
}

impl CameraShake {
    #[allow(dead_code)]
    pub fn add_trauma(&mut self, trauma: f32) {
        if self.trauma == 0.0 {
            self.seed = (SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis()
                & 0xFFFF) as f32;
        }
        self.trauma = (self.trauma + trauma.abs()).min(1.0);
    }

    #[allow(dead_code)]
    pub fn add_trauma_with_threshold(&mut self, trauma: f32, threshold: f32) {
        if self.trauma >= threshold {
            return;
        }
        self.add_trauma(trauma);
    }

    /// Move camera focus target
    pub fn update_target(&mut self, target: Vec2) {
        self.target = target;
    }

    fn reduce_trauma(&mut self, delta: f32) {
        self.trauma = (self.trauma - delta.abs()).max(0.0)
    }

    fn noise_value(&self, stack: u32) -> f32 {
        simplex_noise_2d_seeded(
            Vec2::new(self.trauma * NOISE_STRENGTH, 0.0),
            self.seed + stack as f32,
        )
    }
}

fn decay_shake_trauma(time: Res<Time>, mut shake: ResMut<CameraShake>) {
    shake.reduce_trauma(time.delta_secs());
}

pub fn update_camera(
    mut q_camera: Query<&mut Transform, With<MainCamera>>,
    shake: ResMut<CameraShake>,
) {
    let mut transform = match q_camera.get_single_mut() {
        Ok(t) => t,
        Err(_) => return,
    };

    let translation_offset = Vec3::new(shake.noise_value(0), shake.noise_value(1), 0.0)
        * shake.trauma.powi(2)
        * TRANSLATION_SHAKE_STRENGTH;
    let rotation_offset = Quat::from_rotation_z(
        (shake.noise_value(2) * shake.trauma.powi(2) * ROTATION_SHAKE_STRENGTH).to_radians(),
    );

    transform.translation = shake.target.extend(transform.translation.z) + translation_offset;
    transform.rotation = rotation_offset;
}

fn update_camera_target(mut shake: ResMut<CameraShake>, q_player: Query<&Transform, With<Player>>) {
    let player_transform = match q_player.get_single() {
        Ok(r) => r,
        Err(_) => return,
    };
    shake.update_target(player_transform.translation.truncate());
}
