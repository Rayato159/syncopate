use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    characters::thunwa::Thunwa,
    terrains::{GRID_SIZE, MAP_SIZE, TILE_SIZE},
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum TextureIndex {
    LowerWindow = 262,
    MiddleWinow = 241,
    UpperWindow = 220,
    BloodStain1 = 407,
    BloodStain2 = 408,
    BloodStain3 = 409,
    BloodStain4 = 430,
    BloodStain5 = 452,
    BloodStain6 = 473,
    BloodStain7 = 453,
}

pub fn draw_terrain(mut commands: Commands, asset_server: Res<AssetServer>) {
    let scene_image = asset_server.load("tileset/condo/entering/scene.png");

    commands.spawn((
        Sprite::from_image(scene_image),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // Map bounds
    let width = GRID_SIZE * MAP_SIZE.x as f32;
    let height = GRID_SIZE * MAP_SIZE.y as f32;
    let thickness = 8.;

    // Left
    commands.spawn((
        Collider::cuboid(thickness / 2., height / 2.),
        Transform::from_xyz(-width / 2. - thickness / 2., 0., 0.),
    ));

    // Right
    commands.spawn((
        Collider::cuboid(thickness / 2., height / 2.),
        Transform::from_xyz(width / 2. + thickness / 2., 0., 0.),
    ));

    // Bottom
    commands.spawn((
        Collider::cuboid(width / 2., thickness / 2.),
        Transform::from_xyz(0., -height / 2. - thickness / 2., 0.),
    ));

    // Top
    commands.spawn((
        Collider::cuboid(width / 2., thickness / 2.),
        Transform::from_xyz(0., height / 2. + thickness / 2., 0.),
    ));

    let texture_handle: Handle<Image> = asset_server.load("tileset/condo/entering/tiles_1.png");
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(MAP_SIZE);

    // Front of Upper Floor 1
    commands
        .spawn(Collider::cuboid(
            (GRID_SIZE * 22.) / 2.,
            (GRID_SIZE - 10.) / 2.,
        ))
        .insert(Transform::from_xyz(-GRID_SIZE * 2., GRID_SIZE * 2.7, 0.));

    commands
        .spawn(Collider::cuboid(2., (GRID_SIZE + 10.) / 2.))
        .insert(Transform::from_xyz(-GRID_SIZE * 13., GRID_SIZE * 3.0, 0.));

    // Front of Upper Floor 2
    commands
        .spawn(Collider::cuboid(
            (GRID_SIZE * 11.) / 2.,
            (GRID_SIZE - 10.) / 2.,
        ))
        .insert(Transform::from_xyz(GRID_SIZE * 14.5, GRID_SIZE * 1.7, 0.));

    commands
        .spawn(Collider::cuboid(2., GRID_SIZE / 2.))
        .insert(Transform::from_xyz(GRID_SIZE * 9., GRID_SIZE * 2.0, 0.));

    // Wall
    commands
        .spawn(Collider::cuboid(
            (GRID_SIZE * MAP_SIZE.x as f32) / 2.,
            GRID_SIZE / 2.,
        ))
        .insert(Transform::from_xyz(0., 6.8 * GRID_SIZE, 0.));

    // Lower Window
    draw_windows(
        &mut commands,
        tilemap_entity,
        &mut tile_storage,
        (
            TileTextureIndex(TextureIndex::LowerWindow as u32),
            TileTextureIndex(TextureIndex::MiddleWinow as u32),
            TileTextureIndex(TextureIndex::UpperWindow as u32),
        ),
        (MAP_SIZE.x - 33, MAP_SIZE.x - 3, MAP_SIZE.y - 4),
    );

    draw_bloodstain(&mut commands, tilemap_entity, &mut tile_storage);

    let grid_size = TILE_SIZE.into();
    let map_type = TilemapType::default();

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: MAP_SIZE,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle),
        tile_size: TILE_SIZE,
        anchor: TilemapAnchor::Center,
        ..Default::default()
    });
}

fn draw_bloodstain(
    commands: &mut Commands,
    tilemap_entity: Entity,
    tile_storage: &mut TileStorage,
) {
    let tiles = [
        (0, MAP_SIZE.y - 5, TextureIndex::BloodStain1),
        (1, MAP_SIZE.y - 5, TextureIndex::BloodStain2),
        (2, MAP_SIZE.y - 5, TextureIndex::BloodStain3),
        (2, MAP_SIZE.y - 6, TextureIndex::BloodStain4),
        (3, MAP_SIZE.y - 5, TextureIndex::BloodStain5),
        (3, MAP_SIZE.y - 6, TextureIndex::BloodStain6),
        (4, MAP_SIZE.y - 5, TextureIndex::BloodStain7),
    ];

    for (x, y, stain_index) in tiles {
        let tile_pos = TilePos { x, y };
        let tile_entity = commands
            .spawn(TileBundle {
                position: tile_pos,
                tilemap_id: TilemapId(tilemap_entity),
                texture_index: TileTextureIndex(stain_index as u32),
                ..Default::default()
            })
            .id();
        tile_storage.set(&tile_pos, tile_entity);
    }
}

fn draw_windows(
    commands: &mut Commands,
    tilemap_entity: Entity,
    tile_storage: &mut TileStorage,
    window_texture_index: (TileTextureIndex, TileTextureIndex, TileTextureIndex),
    bounds: (u32, u32, u32),
) {
    let offset = 4;
    let mut x = bounds.0;

    while x < bounds.1 {
        // Lower Window
        for i in 0..2 {
            let tile_pos = TilePos {
                x: x + i,
                y: bounds.2,
            };

            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: window_texture_index.0,
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }

        // Middle Window
        for i in 0..2 {
            let tile_pos = TilePos {
                x: x + i,
                y: bounds.2 + 1,
            };

            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: window_texture_index.1,
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }

        // Upper Window
        for i in 0..2 {
            let tile_pos = TilePos {
                x: x + i,
                y: bounds.2 + 2,
            };

            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: window_texture_index.2,
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }

        x += offset;
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct CondoClosedDoorEntering(pub bool);

pub fn draw_entering_door(mut commands: Commands, asset_server: Res<AssetServer>) {
    let door_image = asset_server.load("tileset/condo/entering/double_door_closed.png");
    let height_adjust = 5. * (GRID_SIZE / 16.);
    let x_collider = 38. * (GRID_SIZE / 16.) / 2.;
    let y_collider = 33. * (GRID_SIZE / 16.) / 2.;

    commands
        .spawn((
            CondoClosedDoorEntering(true),
            Collider::cuboid(x_collider, y_collider),
            Sprite::from_image(door_image),
        ))
        .insert(Transform::from_xyz(
            -(17. * GRID_SIZE as f32),
            (7. * GRID_SIZE as f32) + height_adjust,
            10.0,
        ));
}

pub fn draw_trees(mut commands: Commands, asset_server: Res<AssetServer>) {
    let tree_image = asset_server.load("tileset/condo/entering/tree_1.png");
    let tree_positions = [
        (-GRID_SIZE * 1., -GRID_SIZE * 7.),
        (-GRID_SIZE * 10., -GRID_SIZE * 6.),
        (GRID_SIZE * 6., -GRID_SIZE * 8.5),
        (GRID_SIZE * 13., -GRID_SIZE * 7.),
        (GRID_SIZE * 18., -GRID_SIZE * 8.5),
    ];

    // Colliders
    let x_collider = 10. * (GRID_SIZE / 16.) / 2.;
    let y_collider = 2. * (GRID_SIZE / 16.) / 2.;

    for (x, y) in tree_positions {
        commands
            .spawn((
                DynamicsZOrder,
                Transform::from_xyz(x, y, 30.),
                Sprite::from_image(tree_image.to_owned()),
            ))
            .with_children(|parent| {
                parent
                    .spawn((Collider::cuboid(x_collider, y_collider),))
                    .insert(Transform::from_xyz(0.0, -GRID_SIZE * 1.4, 0.0));
            });
    }
}

pub fn draw_lamps(mut commands: Commands, asset_server: Res<AssetServer>) {
    let lamp_image = asset_server.load("tileset/condo/entering/lamp_1.png");
    let offset = 7.5 * GRID_SIZE;
    let y_position = -GRID_SIZE * 3.5;

    let lamp_positions = [
        // Left
        (-GRID_SIZE * 4. - offset, y_position),
        (-GRID_SIZE * 4., y_position),
        // Right
        (GRID_SIZE * 4., y_position),
        (GRID_SIZE * 4. + offset, y_position),
        (GRID_SIZE * 4. + (offset * 2.), y_position),
    ];

    // Colliders
    let x_collider = 10. * (GRID_SIZE / 16.) / 2.;
    let y_collider = 2. * (GRID_SIZE / 16.) / 2.;

    for (x, y) in lamp_positions {
        commands
            .spawn((
                DynamicsZOrder,
                Sprite::from_image(lamp_image.to_owned()),
                Transform::from_xyz(x, y, 30.),
            ))
            .with_children(|parent| {
                parent
                    .spawn((Collider::cuboid(x_collider, y_collider),))
                    .insert(Transform::from_xyz(0.0, -GRID_SIZE * 1.4, 0.0));
            });
    }
}

#[derive(Component)]
pub struct DynamicsZOrder;

pub fn update_z_order(
    mut sprite_query: Query<&mut Transform, (With<DynamicsZOrder>, Without<Thunwa>)>,
    thunwa_collider_query: Query<&Transform, (With<Thunwa>, Without<DynamicsZOrder>)>,
) {
    if let Ok(collider_transform) = thunwa_collider_query.single() {
        for mut transform in sprite_query.iter_mut() {
            if transform.translation.y - 16. > collider_transform.translation.y {
                transform.translation.z = collider_transform.translation.z - 1.0;
            } else {
                transform.translation.z = 30.;
            }
        }
    }
}
