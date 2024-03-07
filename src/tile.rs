use crate::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Tile;

#[derive(Clone, Component)]
pub enum TileType {
    Wood,
    Stand,
}

impl TileType {
    pub const fn is_walkable(&self) -> bool {
        match self {
            Self::Wood => true,
            Self::Stand => false,
        }
    }
}

#[derive(Bundle)]
pub struct TileBundle {
    pub tile: Tile,
    pub r#type: TileType,
    pub position: MapPosition,
    pub sprite: SpriteSheetBundle,
}