use bevy::prelude::*;

use crate::components::health::Health;

/// Response for heath dmg on screen
pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<HealthBarDisplay>().add_systems(
            Update,
            (
                create_health_bars.before(update_health_bars),
                update_health_bars,
            ),
        );
    }
}

#[derive(Component, Reflect)]
#[require(Mesh2d)]
pub struct HealthBarDisplay;

const HEALTH_COLOR: Color = Color::linear_rgba(1.0, 1.0, 0.0, 1.0);
const MISSING_HEALTH_COLOR: Color = Color::BLACK;

fn create_health_bars(
    mut commands: Commands,
    q_hp: Query<Entity, Added<Health>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for ent in &q_hp {
        let mut ent = commands.entity(ent);
        ent.with_children(|p| {
            // we have 2 entity here

            // spawn hp bar
            p.spawn((
                HealthBarDisplay,
                // 2 mesh ui
                Mesh2d(meshes.add(Rectangle::new(12.0, 2.0))),
                MeshMaterial2d(materials.add(ColorMaterial::from(HEALTH_COLOR))),
                // and relative parent translate
                Transform::from_translation(Vec3::new(0., 10.0, 101.0)), // higher Z
            ));

            // missing hp entity ui
            p.spawn((
                // 2 mesh ui
                Mesh2d(meshes.add(Rectangle::new(12.0, 2.0))),
                MeshMaterial2d(materials.add(ColorMaterial::from(MISSING_HEALTH_COLOR))),
                // and relative parent translate
                Transform::from_translation(Vec3::new(0., 10.0, 100.0)), // lower Z
            ));
        });
    }
}

fn update_health_bars(
    q_hp: Query<&Health>,
    mut q_health_ui: Query<(&Parent, &mut Transform), With<HealthBarDisplay>>,
) {
    for (parent, mut transform) in &mut q_health_ui {
        let entity = parent.get();
        let Ok(hp) = q_hp.get(entity) else {
            // log: weired why we not have hp ???
            continue;
        };

        let percent = hp.percent();
        // Shift the bar's center leftwards as it shrinks
        transform.translation.x = (1.0 - percent) * -6.; // why haft 12 ?
        transform.scale.x = percent;
    }
}
