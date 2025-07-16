use bevy::{prelude::*, window::WindowMode};
use bevy_aseprite_ultra::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_fps_counter::FpsCounterPlugin;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_kira_audio::prelude::*;
use bevy_light_2d::prelude::*;
use bevy_rapier2d::prelude::*;
use syncopate::{
    GameState, PauseState, camera,
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
                    resize_constraints: WindowResizeConstraints {
                        min_width: 640.,
                        min_height: 320.,
                        max_width: 1920.,
                        max_height: 1080.,
                    },
                    mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
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
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(AsepriteUltraPlugin)
        .add_plugins(TilemapPlugin)
        .init_state::<GameState>()
        .init_state::<PauseState>()
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
            )
                .in_set(GameStartUpSet::CondoEntering),
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
            OnExit(GameState::InGame),
            (
                terrains::condo_entering::despawn_condo_entering,
                sounds::condo_entering::stop_playing_soundtrack,
                camera::despawn_player_camera,
                thunwa::despawn_thunwa,
            )
                .in_set(GameUpdateSet::CondoEntering),
        )
        .add_systems(
            Update,
            thunwa::thunwa_camera_following
                .in_set(GameUpdateSet::Thunwa)
                .run_if(in_state(GameState::InGame))
                .run_if(in_state(PauseState::InGame)),
        )
        .add_systems(
            Update,
            thunwa::thunwa_movement
                .in_set(GameUpdateSet::Thunwa)
                .after(GameStartUpSet::Thunwa)
                .run_if(in_state(GameState::InGame))
                .run_if(in_state(PauseState::InGame)),
        )
        .add_systems(
            Update,
            terrains::condo_entering::update_z_order
                .in_set(GameUpdateSet::CondoEntering)
                .after(GameStartUpSet::Thunwa)
                .run_if(in_state(GameState::InGame))
                .run_if(in_state(PauseState::InGame)),
        )
        .add_systems(
            Update,
            ui::paused_menu::paused_menu_toggle.in_set(GameUpdateSet::UI),
        )
        .add_systems(
            OnEnter(PauseState::Paused),
            ui::paused_menu::spawn_paused_menu.in_set(GameStartUpSet::UI),
        )
        .add_systems(
            Update,
            ui::paused_menu::button_pressed_handler
                .in_set(GameUpdateSet::UI)
                .run_if(in_state(PauseState::Paused)),
        )
        .add_systems(
            Update,
            ui::paused_menu::ui_interaction
                .in_set(GameUpdateSet::UI)
                .run_if(in_state(PauseState::Paused)),
        )
        .add_systems(
            OnExit(PauseState::Paused),
            ui::paused_menu::despawn_paused_menu.in_set(GameUpdateSet::UI),
        )
        .run();
}
