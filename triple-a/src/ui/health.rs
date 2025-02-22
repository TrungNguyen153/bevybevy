use bevy::prelude::*;

use crate::components::health::Health;

/// Response for heath dmg on screen
pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<HealthBarDisplay>()
            .register_type::<DamageTextDisplay>()
            .add_systems(
                Update,
                (
                    create_health_bars.before(update_health_bars),
                    update_health_bars,
                    animate_damage_text,
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
    q_hp: Query<(Entity, &Transform), Added<Health>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (ent, t) in &q_hp {
        commands.spawn(DamageTextDisplay::new_bundle(t.translation, 1000., 1.));
        commands.spawn(DamageTextDisplay::new_bundle(t.translation, 500., 1.));
        commands.spawn(DamageTextDisplay::new_bundle(t.translation, 200., 1.));
        commands.spawn(DamageTextDisplay::new_bundle(t.translation, 10., 1.));
        commands.spawn(DamageTextDisplay::new_bundle(t.translation, 300., 1.));
        commands.spawn(DamageTextDisplay::new_bundle(t.translation, 700., 1.));
        commands.spawn(DamageTextDisplay::new_bundle(t.translation, 40., 1.));
        commands.spawn(DamageTextDisplay::new_bundle(t.translation, 20., 1.));
        println!("Welcom");
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
    q_hp: Query<&Health, Or<(Added<Health>, Changed<Health>)>>,
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

//////////////// DMG TEXT //////////////////////

#[derive(Component, Reflect)]
#[require(Text2d)]
pub struct DamageTextDisplay {
    pub display_timer: Timer,
}

impl DamageTextDisplay {
    pub fn new_bundle(translate_pos: Vec3, damage_value: f32, duration_secs: f32) -> impl Bundle {
        (
            Self {
                display_timer: Timer::from_seconds(duration_secs, TimerMode::Once),
            },
            Text2d(format!("{damage_value}")),
            TextColor(Color::linear_rgba(1.0, 0.27, 0.0, 1.0)),
            Transform::from_translation(translate_pos.with_z(100.)),
        )
    }
}

fn animate_damage_text(
    mut commands: Commands,
    mut q_dmg_text: Query<(
        Entity,
        &mut DamageTextDisplay,
        &mut TextColor,
        &mut Transform,
    )>,
    time: Res<Time>,
) {
    for (entity, mut dmg_timer, mut color, mut trans) in &mut q_dmg_text {
        dmg_timer.display_timer.tick(time.delta());

        if dmg_timer.display_timer.just_finished() {
            commands.entity(entity).despawn_recursive();
            println!("Removed dmg text");
            continue;
        }

        // change base on duration finish alpha color
        color.set_alpha(1.0 - dmg_timer.display_timer.fraction().powf(1.5));

        // move text go up
        const MAX_MOVE_Y: f32 = 1.0;
        let changed_y = dmg_timer.display_timer.fraction() * MAX_MOVE_Y;
        trans.translation.y += changed_y;
        println!("Update dmg text: {changed_y}");
    }
}
