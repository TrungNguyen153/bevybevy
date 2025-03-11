#![allow(clippy::too_many_arguments, clippy::type_complexity)]
#![feature(let_chains)]

mod assets;
pub mod components;
pub mod debug;
pub mod events;
pub mod particle;
pub mod player;
pub mod spell;
mod ui;
pub mod world_map;

pub use assets::GameAssets;

use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::window::{PresentMode, Window, WindowMode, WindowResolution};

use avian2d::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_trickfilm::Animation2DPlugin;

use self::debug::mouse_world::MouseWorldPlugin;

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum GameState {
    #[default]
    AssetLoading,
    Gaming,
    GameOver,
    Win,
}

// Root
pub fn create_game() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    watch_for_changes_override: Some(true),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::Fifo,
                        mode: WindowMode::Windowed,
                        fit_canvas_to_parent: false,
                        canvas: Some("#game-canvas".to_string()),
                        resolution: WindowResolution::new(1280.0, 720.0),
                        ..default()
                    }),
                    ..default()
                })
                .build(),
            Animation2DPlugin,
            PhysicsPlugins::default(),
        ))
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .continue_to_state(GameState::Gaming)
                .load_collection::<GameAssets>(),
        )
        .add_plugins((
            components::ComponentPlugin,
            events::EventPlugin,
            ui::UiPlugin,
            world_map::WorldMapPlugin,
            player::PlayerPlugin,
            spell::SpellPlugin,
            MouseWorldPlugin,
            particle::ParticlePlugin,
        ))
        .run();
}
