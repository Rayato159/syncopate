use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::terrains::{GRID_SIZE, MAP_SIZE};

#[derive(Component)]
pub struct Thunwa {
    pub speed: f32,
    pub last_direction: Vec3,
}

#[derive(Component)]
pub struct ThunwaCollider;

pub fn setup_thunwa(mut commands: Commands, asset_server: Res<AssetServer>) {
    let aseprite = asset_server.load("characters/thunwa/thunwa_sprite.aseprite");

    commands
        .spawn((
            Thunwa {
                speed: 160.,
                last_direction: Vec3::ZERO,
            },
            AseAnimation {
                aseprite,
                animation: Animation::tag("idle-front").with_speed(1.),
            },
            Sprite::default(),
            RigidBody::Dynamic,
        ))
        .insert(Velocity::zero())
        .insert(LockedAxes::ROTATION_LOCKED)
        .insert(Transform::from_xyz(
            -((GRID_SIZE * MAP_SIZE.x as f32) / 2. - (GRID_SIZE * 3.)),
            -(GRID_SIZE * 8.),
            20.,
        ))
        .with_children(|parent| {
            parent
                .spawn((ThunwaCollider, Collider::capsule_y(6., 6.)))
                .insert(Transform::from_xyz(0.0, -16.0, 0.));
        });
}

pub fn thunwa_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Thunwa, &mut Velocity, &mut AseAnimation), With<Thunwa>>,
) {
    if let Ok((mut thunwa, mut vel, mut animation)) = query.single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.y = 1.;
            thunwa.last_direction = Vec3::Y;
            animation.animation = Animation::tag("walk-back").with_speed(1.);
        }

        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x = -1.;
            thunwa.last_direction = -Vec3::X;
            animation.animation = Animation::tag("walk-left").with_speed(1.);
        }

        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.y = -1.;
            thunwa.last_direction = -Vec3::Y;
            animation.animation = Animation::tag("walk-front").with_speed(1.);
        }

        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x = 1.;
            thunwa.last_direction = Vec3::X;
            animation.animation = Animation::tag("walk-right").with_speed(1.);
        }

        if direction == Vec3::ZERO {
            vel.linvel = Vec2::ZERO;

            let idle_tag = if thunwa.last_direction == Vec3::Y {
                "idle-back"
            } else if thunwa.last_direction == -Vec3::X {
                "idle-left"
            } else if thunwa.last_direction == Vec3::X {
                "idle-right"
            } else {
                "idle-front"
            };

            animation.animation = Animation::tag(idle_tag).with_speed(1.);
        } else {
            let movement = direction.xy().normalize() * thunwa.speed;
            vel.linvel = movement;
        }
    }
}
