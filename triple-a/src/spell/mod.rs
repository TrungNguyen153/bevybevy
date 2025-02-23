pub mod fireball;

use bevy::prelude::*;

pub struct SpellPlugin;

impl Plugin for SpellPlugin {
    fn build(&self, app: &mut App) {
        //
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum Spell {
    Fireball,
}

#[derive(PartialEq, Debug, Clone)]
pub enum SpellModify {}

#[derive(Component, Reflect)]
pub struct SpellAdditionProjectile(pub usize);

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum DamageType {
    Fire,
    Lightning,
    Cold,
    Poision,
    Chaos,
}

pub trait SpellTrait {
    fn spell_name(&self) -> String;

    fn spell_timer_mut(&mut self) -> &mut Timer;

    fn base_damage(&self) -> f32;

    fn base_critical_change(&self) -> f32;

    fn damamge_type(&self) -> DamageType;
}
