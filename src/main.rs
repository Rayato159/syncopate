use bevy::{prelude::*, window::WindowMode};
use bevy_aseprite_ultra::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_fps_counter::FpsCounterPlugin;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_kira_audio::prelude::*;
use bevy_light_2d::prelude::*;
use bevy_rapier2d::prelude::*;
use syncopate::{
    GameOptions, GameState, MainMenuState, PauseOptionsState, PauseState, camera,
    characters::{
        thunwa::{self, ThunwaHealth},
        zombie,
    },
    pause_physics_system, sounds, terrains,
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
    Zombie,
    CondoEntering,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(GameOptions::default())
        .insert_resource(MainMenuLightFlickerTimer::default())
        .insert_resource(ThunwaHealth::default())
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Syncopate".into(),
                    resizable: true,
                    resize_constraints: WindowResizeConstraints {
                        min_width: 1280.,
                        min_height: 640.,
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
        .init_state::<MainMenuState>()
        .init_state::<PauseState>()
        .init_state::<PauseOptionsState>()
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
                camera::main_menu_camera_setup,
                sounds::main_menu::play_soundtrack,
                ui::main_menu::spawn_main_menu_scene,
            )
                .before(GameStartUpSet::UI),
        )
        .add_systems(
            OnEnter(MainMenuState::MainMenu),
            ui::main_menu::spawn_main_menu.in_set(GameStartUpSet::UI),
        )
        .add_systems(
            OnExit(MainMenuState::MainMenu),
            ui::main_menu::despawn_main_menu.in_set(GameUpdateSet::UI),
        )
        .add_systems(
            OnEnter(MainMenuState::Options),
            ui::options::spawn_options_menu.in_set(GameStartUpSet::UI),
        )
        .add_systems(
            Update,
            (
                ui::screen_mode_button_handler,
                ui::options::music_volume_button_handler,
                ui::options::back_by_keyboard_input_handler,
            )
                .in_set(GameUpdateSet::UI)
                .run_if(in_state(GameState::MainMenu))
                .run_if(in_state(MainMenuState::Options)),
        )
        .add_systems(
            OnExit(MainMenuState::Options),
            ui::options::despawn_options_menu.in_set(GameUpdateSet::UI),
        )
        .add_systems(
            OnExit(GameState::MainMenu),
            (
                sounds::main_menu::stop_playing_soundtrack,
                ui::main_menu::despawn_main_menu_scene,
                ui::main_menu::despawn_main_menu,
                camera::despawn_main_menu_camera,
            )
                .in_set(GameUpdateSet::UI),
        )
        .add_systems(
            Update,
            ui::main_menu::light_flicker
                .in_set(GameUpdateSet::UI)
                .run_if(in_state(GameState::MainMenu)),
        )
        .add_systems(Update, ui::ui_interaction.in_set(GameUpdateSet::UI))
        .add_systems(
            Update,
            ui::main_menu::button_pressed_handler
                .in_set(GameUpdateSet::UI)
                .run_if(in_state(GameState::MainMenu))
                .run_if(in_state(MainMenuState::MainMenu)),
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
            (
                thunwa::setup_thunwa,
                zombie::setup_zombies,
                camera::player_camera_setup,
            )
                .in_set(GameStartUpSet::Thunwa),
        )
        .add_systems(
            OnExit(GameState::InGame),
            (
                terrains::condo_entering::despawn_condo_entering,
                sounds::condo_entering::stop_playing_soundtrack,
                camera::despawn_player_camera,
                thunwa::despawn_thunwa,
                zombie::despawn_zombies,
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
            (
                zombie::update_zombie_ai,
                zombie::zombie_attack_system,
                zombie::update_zombie_animation_direction,
                zombie::update_player_health_display,
            )
                .in_set(GameUpdateSet::Zombie)
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
            ui::paused_menu::pause_handler
                .in_set(GameUpdateSet::UI)
                .run_if(in_state(PauseState::InGame))
                .run_if(in_state(PauseOptionsState::None)),
        )
        .add_systems(
            Update,
            pause_physics_system.run_if(in_state(GameState::InGame)),
        )
        .add_systems(
            Update,
            ui::paused_menu::un_pause_handler
                .in_set(GameUpdateSet::UI)
                .run_if(in_state(PauseState::Paused))
                .run_if(in_state(PauseOptionsState::Paused)),
        )
        .add_systems(
            Update,
            ui::paused_menu::button_pressed_handler
                .in_set(GameUpdateSet::UI)
                .run_if(in_state(PauseState::Paused))
                .run_if(in_state(PauseOptionsState::Paused)),
        )
        .add_systems(
            OnEnter(PauseOptionsState::Paused),
            ui::paused_menu::spawn_paused_menu.in_set(GameStartUpSet::UI),
        )
        .add_systems(
            OnExit(PauseOptionsState::Paused),
            ui::paused_menu::despawn_paused_menu.in_set(GameUpdateSet::UI),
        )
        .add_systems(
            OnEnter(PauseOptionsState::Options),
            (ui::in_game_options_menu::spawn_paused_options_menu,).in_set(GameStartUpSet::UI),
        )
        .add_systems(
            OnExit(PauseOptionsState::Options),
            (ui::in_game_options_menu::despawn_paused_options_menu,).in_set(GameUpdateSet::UI),
        )
        .add_systems(
            Update,
            (
                ui::screen_mode_button_handler,
                ui::in_game_options_menu::music_volume_button_handler,
                ui::in_game_options_menu::back_to_options_handler,
            )
                .in_set(GameUpdateSet::UI)
                .run_if(in_state(PauseOptionsState::Options)),
        )
        .run();
}
