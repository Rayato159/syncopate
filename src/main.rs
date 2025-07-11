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
    characters::thunwa,
    sounds, terrains,
    ui::{self, main_menu::MainMenuLightFlickerTimer},
};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameStartUpSet {
    UI,
    Physics,
    Camera,
    Thunwa,
    CondoEntering,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameUpdateSet {
    UI,
    Camera,
    Thunwa,
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
                GameStartUpSet::Camera,
            )
                .chain(),
        )
        .configure_sets(
            Update,
            (
                GameUpdateSet::CondoEntering,
                GameUpdateSet::Thunwa,
                GameUpdateSet::Camera,
            )
                .chain(),
        )
        .add_systems(
            OnEnter(GameState::MainMenu),
            (
                ui::main_menu::spawn_main_menu,
                camera::main_menu_camera_setup,
                sounds::main_menu::play_soundtrack,
            )
                .in_set(GameStartUpSet::UI),
        )
        .add_systems(
            OnExit(GameState::MainMenu),
            (
                sounds::main_menu::stop_playing_soundtrack,
                ui::main_menu::despawn_main_menu,
            )
                .in_set(GameUpdateSet::UI),
        )
        .add_systems(
            Update,
            ui::main_menu::light_flicker
                .in_set(GameUpdateSet::UI)
                .run_if(in_state(GameState::MainMenu)),
        )
        .add_systems(
            Update,
            ui::main_menu::ui_interaction
                .in_set(GameUpdateSet::UI)
                .run_if(in_state(GameState::MainMenu)),
        )
        .add_systems(
            Update,
            ui::main_menu::button_pressed_handler
                .in_set(GameUpdateSet::UI)
                .run_if(in_state(GameState::MainMenu)),
        )
        .add_systems(
            OnEnter(GameState::InGame),
            syncopate::global_bevy_rapier_config.in_set(GameStartUpSet::Physics),
        )
        .add_systems(
            OnEnter(GameState::InGame),
            (
                sounds::condo_entering::play_soundtrack,
                terrains::condo_entering::draw_terrain,
                terrains::condo_entering::draw_entering_door,
                terrains::condo_entering::draw_trees,
                terrains::condo_entering::draw_lamps,
            )
                .in_set(GameStartUpSet::CondoEntering),
        )
        .add_systems(
            OnExit(GameState::InGame),
            sounds::condo_entering::stop_playing_soundtrack.in_set(GameUpdateSet::CondoEntering),
        )
        .add_systems(
            OnEnter(GameState::InGame),
            thunwa::setup_thunwa.in_set(GameStartUpSet::Thunwa),
        )
        .add_systems(
            OnEnter(GameState::InGame),
            camera::player_camera_setup.in_set(GameStartUpSet::Thunwa),
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
            terrains::condo_entering::update_z_order
                .in_set(GameUpdateSet::CondoEntering)
                .after(GameStartUpSet::Thunwa)
                .run_if(in_state(GameState::InGame)),
        )
        .run();
}
