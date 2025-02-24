pub mod fireball;

use std::time::Duration;

use bevy::prelude::*;
use bevy::{sprite::Material2dPlugin, utils::hashbrown::HashMap};

use crate::{GameAssets, GameState, components::movement::Facing, player::Player};

use self::fireball::{CustomFireballShader, spawn_fireball};

pub struct SpellPlugin;

impl Plugin for SpellPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SpellModAdditionProjectile>()
            .register_type::<SpellModAdditionPiercing>()
            .register_type::<SpellModTravelSpeed>()
            .register_type::<SpellModSpellEcho>()
            .register_type::<SpellModSpellMoreCritical>()
            .register_type::<SpellCaster>()
            .register_type::<SpellModReduceCooldown>()
            .add_event::<CastSpellEvent>()
            // load shader fireball
            .add_plugins(Material2dPlugin::<CustomFireballShader>::default())
            .add_systems(
                Update,
                (
                    player_auto_casting,
                    spawn_spell_projectiles,
                    fireball::update_fireball,
                )
                    .run_if(in_state(GameState::Gaming)),
            );
    }
}

#[derive(Reflect, Eq, Hash, PartialEq, Debug, Clone)]
pub enum Spell {
    Fireball,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Reflect)]
pub enum DamageType {
    Fire,
    Lightning,
    Cold,
    Poision,
    Chaos,
}

#[derive(Component, Reflect, Default)]
pub struct SpellCaster {
    cooldown_pool: HashMap<Spell, Timer>,
}

// Spells Modifiles
#[derive(Component, Reflect)]
pub struct SpellModAdditionProjectile(pub usize);
#[derive(Component, Reflect)]
pub struct SpellModAdditionPiercing(pub usize);
#[derive(Component, Reflect)]
pub struct SpellModTravelSpeed(pub f32);
#[derive(Component, Reflect)]
pub struct SpellModReduceCooldown(pub f32);
#[derive(Component, Reflect)]
pub struct SpellModSpellEcho;
#[derive(Component, Reflect)]
pub struct SpellModSpellMoreCritical(pub f32);

#[derive(Event)]
pub struct CastSpellEvent {
    pub belong: Entity,
    pub spell: Spell,
}

fn spawn_spell_projectiles(
    mut command: Commands,
    mut ev: EventReader<CastSpellEvent>,
    mut q: Query<
        (
            Option<&SpellModAdditionProjectile>,
            Option<&SpellModAdditionPiercing>,
            Option<&SpellModTravelSpeed>,
            Option<&SpellModReduceCooldown>,
            Option<&SpellModSpellMoreCritical>,
            Has<SpellModSpellEcho>,
            &Facing,
            &Transform,
            &mut SpellCaster,
        ),
        With<SpellCaster>,
    >,
    time: Res<Time>,
    assets: Res<GameAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomFireballShader>>,
) {
    let delta = time.delta();

    for CastSpellEvent { belong, spell } in ev.read() {
        #[rustfmt::skip]
        let Ok((projectiles,
                piering,
                speed,
                reduce_cd,
                more_critical,
                has_echo,
                facing,
                trasform,
                mut caster))
            = q.get_mut(*belong)
        else {
            println!("Not found caster ?");
            continue;
        };

        let mut is_cd = false;

        match caster.cooldown_pool.get_mut(spell) {
            Some(e) => {
                e.tick(delta);
                is_cd = !e.just_finished();
            }
            None => {
                // non exist
            }
        }

        const BASE_COOLDOWN_IN_SECS: f32 = 5.;
        const BASE_SPELL_CRITICAL: f32 = 5.;
        const BASE_SPELL_PROJECTILES: usize = 1;
        const BASE_SPELL_PIERCING: usize = 0;

        if is_cd {
            continue;
        }

        // reset cd
        // calculate cooldown
        let mut cd = BASE_COOLDOWN_IN_SECS;
        if let Some(reduce_cd) = reduce_cd {
            cd -= reduce_cd.0;
        }

        cd = cd.clamp(0.1, 10.);

        caster
            .cooldown_pool
            .entry(spell.clone())
            .insert(Timer::new(Duration::from_secs_f32(cd), TimerMode::Once));

        let mut critical_change = BASE_SPELL_CRITICAL;

        if let Some(more_critical) = more_critical {
            critical_change *= more_critical.0;
            critical_change = critical_change.clamp(0., 100.);
        }

        let mut projectiles_count = BASE_SPELL_PROJECTILES;
        if let Some(projectiles) = projectiles {
            projectiles_count += projectiles.0;
        }

        let mut piercing_count = BASE_SPELL_PIERCING;
        if let Some(p) = piering {
            piercing_count += p.0;
        }

        // now spawn spell
        spawn_fireball(
            &mut command,
            &assets,
            &mut meshes,
            &mut materials,
            trasform.translation,
            facing,
            critical_change,
            projectiles_count,
            piercing_count,
        );
    }
}

fn player_auto_casting(mut command: Commands, q_player: Query<Entity, With<Player>>) {
    for player in &q_player {
        command.send_event(CastSpellEvent {
            belong: player,
            spell: Spell::Fireball,
        });
    }
}
