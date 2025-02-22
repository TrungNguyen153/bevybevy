use std::time::Duration;

use bevy::prelude::*;

#[derive(Component, Reflect)]
pub struct AnimationSpriteRepeat;

#[derive(Component, Reflect, Deref, DerefMut)]
pub struct FrameTimer(pub Timer);

impl FrameTimer {
    pub fn new(fps: u8) -> Self {
        FrameTimer(Timer::new(
            Duration::from_secs_f32(1.0 / (fps as f32)),
            TimerMode::Once,
        ))
    }
}

#[derive(Component, Reflect)]
#[require(Sprite)]
pub struct AnimationSpriteIndices {
    pub first: usize,
    pub last: usize,
    /// insc atlas index by 1 each this fps value
    pub fps: u8,
    pub current: usize,
    pub frame_timer: FrameTimer,
}

impl AnimationSpriteIndices {
    pub fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first,
            last,
            fps,
            current: 0,
            frame_timer: FrameTimer::new(fps),
        }
    }

    pub fn new_with_repeat_bundle(first: usize, last: usize, fps: u8) -> impl Bundle {
        (Self::new(first, last, fps), AnimationSpriteRepeat)
    }
}

pub fn execute_animate_sprites(
    time: Res<Time>,
    mut query: Query<(
        &mut AnimationSpriteIndices,
        &mut Sprite,
        Option<&AnimationSpriteRepeat>,
    )>,
) {
    for (mut state, mut sprite, want_repeat) in query.iter_mut() {
        state.frame_timer.tick(time.delta());
        if state.frame_timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index == state.last {
                    if want_repeat.is_some() {
                        atlas.index = state.first;
                        state.frame_timer.reset();
                        state.frame_timer.unpause();
                    }
                } else if atlas.index < state.last {
                    atlas.index += 1;
                    state.frame_timer.reset();
                    state.frame_timer.unpause();
                } else {
                    panic!("Why atlas index bigger than AnimationSpriteIndices last index ?")
                }

                state.current = atlas.index;
            }
        }
    }
}
