use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::terrains::{DynamicsZOrder, GRID_SIZE, MAP_SIZE};

#[derive(Component)]
pub struct Ravissara {
    pub speed: f32,
    pub last_direction: Vec3,
}

#[derive(Component)]
pub struct RavissaraCollider;

pub fn setup_ravissara(mut commands: Commands, asset_server: Res<AssetServer>) {
    let aseprite = asset_server.load("characters/ravissara/ravissara_sprite.aseprite");

    commands
        .spawn((
            Ravissara {
                speed: 160.,
                last_direction: Vec3::ZERO,
            },
            DynamicsZOrder,
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
            -((GRID_SIZE * MAP_SIZE.x as f32) / 2. - (GRID_SIZE * 5.)),
            -(GRID_SIZE * 8.),
            20.,
        ))
        .with_children(|parent| {
            parent
                .spawn((RavissaraCollider, Collider::capsule_y(6., 6.)))
                .insert(Transform::from_xyz(0.0, -16.0, 0.));
        });
}

pub fn ravissara_movement(
    mut query: Query<(&mut Ravissara, &mut Velocity, &mut AseAnimation), With<Ravissara>>,
) {
    if let Ok((_, mut vel, _)) = query.single_mut() {
        vel.linvel = Vec2::ZERO;
    }
}
