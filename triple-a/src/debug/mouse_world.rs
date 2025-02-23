use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::world_map::camera::MainCamera;
pub struct MouseWorldPlugin;

impl Plugin for MouseWorldPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CursorPosInfo>()
            .init_resource::<CursorPosInfo>()
            .add_systems(
                Update,
                (
                    update_mouse_world.before(update_mouse_world_display),
                    update_mouse_world_display,
                ),
            );
    }
}

#[derive(Resource, Reflect, Default)]
struct CursorPosInfo {
    world_pos: Vec3,
    rel_window_pos: Vec2,
}

fn update_mouse_world(
    // query to get the window (so we can read the current cursor position)
    q_window: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut cursor_data: ResMut<CursorPosInfo>,
) {
    // get the camera info and transform
    let Ok((camera, camera_transform)) = q_camera.get_single() else {
        return;
    };

    // There is only one primary window
    let Ok(window) = q_window.get_single() else {
        return;
    };

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    let Some(relative) = window.cursor_position() else {
        return;
    };

    let Ok(world_pos) = camera.viewport_to_world(camera_transform, relative) else {
        return;
    };

    cursor_data.rel_window_pos = relative;
    cursor_data.world_pos = world_pos.origin;
}

#[derive(Component)]
#[require(Text2d)]
struct MouseWorldPosTextDisplay;

fn update_mouse_world_display(
    mut cmd: Commands,
    mut q_text: Query<(&mut Transform, &mut Text2d), With<MouseWorldPosTextDisplay>>,
    cursor_data: Res<CursorPosInfo>,
) {
    let translation = cursor_data.world_pos.with_z(200.);
    let Vec3 { x, y, .. } = cursor_data.world_pos;
    let text_display = format!("{x}, {y}");
    match q_text.get_single_mut() {
        Ok((mut transform, mut text)) => {
            // update it
            **text = text_display;
            transform.translation = translation;
        }
        Err(_) => {
            // create one
            cmd.spawn((
                MouseWorldPosTextDisplay,
                Text2d(text_display),
                // TextFont {
                //     font_size: 8.,
                //     ..Default::default()
                // },
                Transform::from_translation(translation),
            ));
        }
    }
}
