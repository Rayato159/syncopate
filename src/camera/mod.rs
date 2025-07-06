use bevy::prelude::*;

pub fn camera_setup(mut commands: Commands) {
    commands.spawn((Camera2d, Transform::from_xyz(0.0, 0.0, 1000.0)));
}
