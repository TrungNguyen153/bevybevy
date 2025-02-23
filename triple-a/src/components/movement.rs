use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(Component, Reflect, Debug, Clone, Default, Deref, DerefMut)]
pub struct Velocity(pub Vec3);

impl Velocity {
    pub fn from_direction_speed(direction: Vec3, speed: f32) -> Self {
        Self(direction.normalize_or_zero() * speed)
    }

    pub fn change_direction(&mut self, direction: Vec3) {
        let speed = self.length();
        self.change_direction_speed(direction, speed);
    }

    pub fn change_direction_speed(&mut self, direction: Vec3, speed: f32) {
        self.0 = direction.normalize_or_zero() * speed;
    }
}

#[derive(Component, Reflect, Debug, Clone, Default, Deref, DerefMut)]
pub struct Facing(pub Vec3);

pub fn update_linear_velocity(mut q_velo: Query<(&Velocity, &mut LinearVelocity)>) {
    for (v, mut physical_velo) in &mut q_velo {
        physical_velo.x = v.x;
        physical_velo.y = v.y;
    }
}

pub fn update_sprite_direction(mut q: Query<(&Facing, &mut Sprite)>) {
    for (facing, mut sprite) in &mut q {
        sprite.flip_x = facing.x.signum() < 0.;
    }
}
