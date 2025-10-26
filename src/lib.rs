use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub mod camera;
pub mod characters;
pub mod sounds;
pub mod terrains;
pub mod ui;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
    Paused,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum MainMenuState {
    None,
    #[default]
    MainMenu,
    Options,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PauseState {
    #[default]
    None,
    InGame,
    Paused,
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PauseOptionsState {
    #[default]
    None,
    Paused,
    Options,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WindowModeSelection {
    Fullscreen,
    Windowed,
}

#[derive(Resource, Clone, Debug)]
pub struct GameOptions {
    pub window_mode: WindowModeSelection,
    pub music_volume: f64,
}

impl Default for GameOptions {
    fn default() -> Self {
        Self {
            window_mode: WindowModeSelection::Fullscreen,
            music_volume: 1.0,
        }
    }
}

pub fn global_bevy_rapier_config(mut rapier_config: Query<&mut RapierConfiguration>) {
    if let Ok(mut rapier_config) = rapier_config.single_mut() {
        rapier_config.gravity = Vec2::ZERO;
    }
}

pub fn pause_physics_system(
    pause_state: Res<State<PauseState>>,
    mut rapier_config: Query<&mut RapierConfiguration>,
) {
    if let Ok(mut rapier_config) = rapier_config.single_mut() {
        match pause_state.get() {
            PauseState::InGame => {
                // Game is running, enable physics
                if !rapier_config.physics_pipeline_active {
                    println!("🎮 Game resumed - Physics enabled");
                }
                rapier_config.physics_pipeline_active = true;
            }
            PauseState::Paused => {
                // Game is paused, disable physics
                if rapier_config.physics_pipeline_active {
                    println!("⏸️  Game paused - Physics disabled");
                }
                rapier_config.physics_pipeline_active = false;
            }
            _ => {}
        }
    }
}
