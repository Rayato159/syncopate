use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub mod camera;
pub mod characters;
pub mod terrains;
pub mod ui;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
    Paused,
}

pub fn global_bevy_rapier_config(
    mut rapier_config: Query<&mut RapierConfiguration>,
    // other params...
) {
    if let Ok(mut rapier_config) = rapier_config.single_mut() {
        rapier_config.gravity = Vec2::ZERO;
    }
}
