use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_fps_counter::FpsCounterPlugin;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_kira_audio::prelude::*;
use bevy_light_2d::prelude::*;
use bevy_rapier2d::prelude::*;
use syncopate::{
    GameState, camera,
    characters::{ravissara, thunwa},
    terrains,
    ui::{self, main_menu::MainMenuLightFlickerTimer},
};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameStartUpSet {
    UI,
    Physics,
    Camera,
    Thunwa,
    Ravissara,
    CondoEntering,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameUpdateSet {
    Camera,
    Thunwa,
    Ravissara,
    CondoEntering,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(MainMenuLightFlickerTimer::default())
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Syncopate".into(),
                    resizable: true,
                    ..Default::default()
                }),
                ..Default::default()
            }),
            Light2dPlugin,
            AudioPlugin,
        ))
        .add_plugins(FpsCounterPlugin)
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        // .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        // .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(AsepriteUltraPlugin)
        .add_plugins(TilemapPlugin)
        .init_state::<GameState>()
        .configure_sets(
            Startup,
            (
                GameStartUpSet::UI,
                GameStartUpSet::Physics,
                GameStartUpSet::CondoEntering,
                GameStartUpSet::Thunwa,
                GameStartUpSet::Ravissara,
                GameStartUpSet::Camera,
            )
                .chain(),
        )
        .configure_sets(
            Update,
            (
                GameUpdateSet::CondoEntering,
                GameUpdateSet::Thunwa,
                GameUpdateSet::Ravissara,
                GameUpdateSet::Camera,
            )
                .chain(),
        )
        .add_systems(
            OnEnter(GameState::MainMenu),
            ui::main_menu::spawn_main_menu.in_set(GameStartUpSet::UI),
        )
        .add_systems(
            OnEnter(GameState::MainMenu),
            camera::main_menu_camera_setup.in_set(GameStartUpSet::Camera),
        )
        .add_systems(
            Update,
            ui::main_menu::light_flicker
                .in_set(GameUpdateSet::Camera)
                .run_if(in_state(GameState::MainMenu)),
        )
        .add_systems(
            OnEnter(GameState::InGame),
            syncopate::global_bevy_rapier_config.in_set(GameStartUpSet::Physics),
        )
        .add_systems(
            OnEnter(GameState::InGame),
            terrains::condo_entering::draw_terrain.in_set(GameStartUpSet::CondoEntering),
        )
        .add_systems(
            OnEnter(GameState::InGame),
            terrains::condo_entering::draw_entering_door.in_set(GameStartUpSet::CondoEntering),
        )
        .add_systems(
            OnEnter(GameState::InGame),
            terrains::condo_entering::draw_trees.in_set(GameStartUpSet::CondoEntering),
        )
        .add_systems(
            OnEnter(GameState::InGame),
            terrains::condo_entering::draw_lamps.in_set(GameStartUpSet::CondoEntering),
        )
        .add_systems(
            OnEnter(GameState::InGame),
            terrains::condo_entering::play_soundtrack.in_set(GameStartUpSet::CondoEntering),
        )
        .add_systems(
            OnEnter(GameState::InGame),
            camera::player_camera_setup.in_set(GameStartUpSet::CondoEntering),
        )
        .add_systems(
            OnEnter(GameState::InGame),
            thunwa::setup_thunwa.in_set(GameStartUpSet::Thunwa),
        )
        .add_systems(
            OnEnter(GameState::InGame),
            ravissara::setup_ravissara
                .in_set(GameStartUpSet::Ravissara)
                .after(GameStartUpSet::Thunwa),
        )
        .add_systems(
            Update,
            thunwa::thunwa_camera_following
                .in_set(GameUpdateSet::Thunwa)
                .run_if(in_state(GameState::InGame)),
        )
        .add_systems(
            Update,
            thunwa::thunwa_movement
                .in_set(GameUpdateSet::Thunwa)
                .after(GameStartUpSet::Thunwa)
                .run_if(in_state(GameState::InGame)),
        )
        .add_systems(
            Update,
            ravissara::ravissara_movement
                .in_set(GameUpdateSet::Ravissara)
                .after(GameStartUpSet::Ravissara)
                .run_if(in_state(GameState::InGame)),
        )
        .add_systems(
            Update,
            terrains::condo_entering::update_z_order
                .in_set(GameUpdateSet::CondoEntering)
                .after(GameStartUpSet::Thunwa)
                .run_if(in_state(GameState::InGame)),
        )
        .run();
}
