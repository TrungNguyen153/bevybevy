use avian2d::prelude::*;
use bevy::prelude::*;
use bevy::{
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{AlphaMode2d, Material2d},
};

use crate::{GameAssets, components::movement::Facing};

use super::DamageType;

#[derive(Component, Reflect)]
// #[require(Sprite)]
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
    mut meshes: &mut ResMut<Assets<Mesh>>,
    mut materials: &mut ResMut<Assets<CustomFireballShader>>,
    spawn_position: Vec3,
    facing: &Facing,
    critical_change: f32,
    projectiles: usize,
    piercing_count: usize,
) {
    for _ in 0..projectiles {
        const CANVAS_WIDTH: f32 = 400.;
        const CANVAS_HEIGHT: f32 = 200.;
        command.spawn((
            // Sprite {
            //     image: assets.fireball_texture.clone(),
            //     texture_atlas: Some(TextureAtlas {
            //         layout: assets.fireball_layout.clone(),
            //         ..Default::default()
            //     }),
            //     ..default()
            // },
            Sensor,
            Fireball {
                timelife: Timer::from_seconds(2., TimerMode::Once),
                base_damage: 10.,
                damage_type: DamageType::Fire,
                critical_change,
                piercing_count,
            },
            LinearVelocity(facing.truncate() * 40.),
            Mesh2d(meshes.add(Rectangle::new(CANVAS_WIDTH, CANVAS_HEIGHT))),
            MeshMaterial2d(materials.add(CustomFireballShader {
                width: CANVAS_WIDTH,
                height: CANVAS_HEIGHT,
            })),
            RigidBody::Dynamic,
            Collider::circle(5.),
            Transform::from_translation(spawn_position.with_z(200.)),
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

/// This example uses a shader source file from the assets subdirectory

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomFireballShader {
    #[uniform(0)]
    width: f32,
    #[uniform(1)]
    height: f32,
}
const SHADER_ASSET_PATH: &str = "shaders/fireball.wgsl";
impl Material2d for CustomFireballShader {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Mask(0.5)
    }
}
