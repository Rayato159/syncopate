use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

// Notes
// X: 0 Is left
// Y: 0 Is bottom

pub(crate) const GRID_SIZE: f32 = 32.0;
pub(crate) const TILE_SIZE: TilemapTileSize = TilemapTileSize {
    x: GRID_SIZE,
    y: GRID_SIZE,
};
pub(crate) const MAP_SIZE: TilemapSize = TilemapSize { x: 40, y: 20 };

#[derive(Component)]
pub struct DynamicsZOrder;

pub mod condo_entering;
