use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_fps_counter::FpsCounterPlugin;
use bevy_inspector_egui::bevy_egui::EguiPlugin;
use bevy_light_2d::prelude::*;
use bevy_rapier2d::prelude::*;
use syncopate::{camera, characters::thunwa, terrains};

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameStartUpSet {
    Physics,
    Camera,
    Thunwa,
    CondoEntering,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameUpdateSet {
    Camera,
    Thunwa,
    CondoEntering,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
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
        .configure_sets(
            Startup,
            (
                GameStartUpSet::Physics,
                GameStartUpSet::Camera,
                GameStartUpSet::CondoEntering,
                GameStartUpSet::Thunwa,
            )
                .chain(),
        )
        .configure_sets(
            Update,
            (
                GameUpdateSet::Camera,
                GameUpdateSet::CondoEntering,
                GameUpdateSet::Thunwa,
            )
                .chain(),
        )
        .add_systems(
            Startup,
            syncopate::global_bevy_rapier_config.in_set(GameStartUpSet::Physics),
        )
        .add_systems(
            Startup,
            terrains::condo_entering::draw_terrain.in_set(GameStartUpSet::CondoEntering),
        )
        .add_systems(
            Startup,
            terrains::condo_entering::draw_entering_door.in_set(GameStartUpSet::CondoEntering),
        )
        .add_systems(
            Startup,
            terrains::condo_entering::draw_trees.in_set(GameStartUpSet::CondoEntering),
        )
        .add_systems(
            Startup,
            terrains::condo_entering::draw_lamps.in_set(GameStartUpSet::CondoEntering),
        )
        .add_systems(
            Startup,
            camera::camera_setup.in_set(GameStartUpSet::CondoEntering),
        )
        .add_systems(Startup, thunwa::setup_thunwa.in_set(GameStartUpSet::Thunwa))
        .add_systems(
            Update,
            camera::thunwa_camera_following
                .in_set(GameUpdateSet::Camera)
                .after(GameStartUpSet::Thunwa),
        )
        .add_systems(
            Update,
            thunwa::thunwa_movement
                .in_set(GameUpdateSet::Thunwa)
                .after(GameStartUpSet::Thunwa),
        )
        .add_systems(
            Update,
            terrains::condo_entering::update_z_order
                .in_set(GameUpdateSet::CondoEntering)
                .after(GameStartUpSet::Thunwa),
        )
        .run();
}
