use bevy::{prelude::*, render::camera::ScalingMode};

use crate::characters::thunwa::Thunwa;

#[derive(Component)]
pub struct PlayerCamera;

pub fn camera_setup(mut commands: Commands) {
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

pub fn thunwa_camera_following(
    mut query: Query<(&mut Transform, Option<&Thunwa>), With<PlayerCamera>>,
    thunwa_query: Query<&Transform, (With<Thunwa>, Without<PlayerCamera>)>,
) {
    if let Ok(thunwa_transform) = thunwa_query.single() {
        if let Ok((mut camera_transform, _)) = query.single_mut() {
            camera_transform.translation.x = thunwa_transform.translation.x;
            camera_transform.translation.y = thunwa_transform.translation.y;
        }
    }
}
