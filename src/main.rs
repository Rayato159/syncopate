use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};
use bevy_rapier2d::prelude::*;
use syncopate::{camera, terrains};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin {
            enable_multipass_for_primary_context: true,
        })
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(TilemapPlugin)
        .add_systems(Startup, terrains::condo::draw_condo_entering_tiles)
        .add_systems(Startup, terrains::condo::draw_condo_entering_door)
        .add_systems(Startup, terrains::condo::draw_trees)
        .add_systems(Startup, terrains::condo::draw_lamps)
        .add_systems(Startup, camera::camera_setup)
        .run();
}
