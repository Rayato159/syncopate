use bevy::{prelude::*, render::camera::ScalingMode};

#[derive(Component)]
pub struct PlayerCamera;

pub fn player_camera_setup(mut commands: Commands) {
    commands.spawn((
        PlayerCamera,
        Camera2d,
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 640.,
            },
            scale: 0.7,
            ..OrthographicProjection::default_2d()
        }),
        Transform::from_xyz(0.0, 0.0, 1000.),
    ));
}

pub fn despawn_player_camera(
    mut commands: Commands,
    player_camera_query: Query<Entity, With<PlayerCamera>>,
) {
    for entity in player_camera_query.iter() {
        commands.entity(entity).despawn();
    }
}

#[derive(Component)]
pub struct MainMenuCamera;

pub fn main_menu_camera_setup(mut commands: Commands) {
    commands.spawn((MainMenuCamera, Camera2d));
}
