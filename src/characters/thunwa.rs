use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    camera::PlayerCamera,
    terrains::{GRID_SIZE, MAP_SIZE},
};

// Collision groups for physics
const PLAYER_COLLISION_GROUP: u32 = 0b0010;
const WALL_COLLISION_GROUP: u32 = 0b0100;
const ZOMBIE_COLLISION_GROUP: u32 = 0b0001;

#[derive(Resource)]
pub struct ThunwaHealth {
    pub current: f32,
    pub max: f32,
}

impl Default for ThunwaHealth {
    fn default() -> Self {
        ThunwaHealth {
            current: 100.0,
            max: 100.0,
        }
    }
}

#[derive(Component)]
pub struct Thunwa {
    pub speed: f32,
    pub last_direction: Vec3,
}

#[derive(Component)]
pub struct ThunwaCollider;

pub fn setup_thunwa(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut thunwa_health: ResMut<ThunwaHealth>,
) {
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
                .spawn((
                    ThunwaCollider,
                    Collider::capsule_y(6., 6.),
                    CollisionGroups::new(
                        Group::from_bits(PLAYER_COLLISION_GROUP).unwrap(),
                        Group::from_bits(ZOMBIE_COLLISION_GROUP | WALL_COLLISION_GROUP).unwrap(),
                    ),
                    ActiveEvents::COLLISION_EVENTS,
                ))
                .insert(Transform::from_xyz(0.0, -16.0, 0.));
        });

    thunwa_health.current = thunwa_health.max;
}

pub fn despawn_thunwa(mut commands: Commands, thunwa_query: Query<Entity, With<Thunwa>>) {
    for entity in thunwa_query.iter() {
        commands.entity(entity).despawn();
    }
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
