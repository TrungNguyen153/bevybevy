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

pub fn update_position(mut q_transform: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for (v, mut t) in &mut q_transform {
        t.translation += v.0 * time.delta_secs();
    }
}

pub fn update_sprite_direction(mut q: Query<(&Facing, &mut Sprite)>) {
    for (facing, mut sprite) in &mut q {
        sprite.flip_x = facing.x.signum() < 0.;
    }
}
