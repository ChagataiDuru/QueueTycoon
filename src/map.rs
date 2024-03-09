use crate::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            tiles: vec![TileType::Stand; width * height],
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Option<&TileType> {
        self.tiles.get(y * self.width + x)
    }

    pub fn get_tile_mut(&mut self, x: usize, y: usize) -> Option<&mut TileType> {
        self.tiles.get_mut(y * self.width + x)
    }
}

#[derive(Component, Debug)]
pub struct MapPosition {
    pub x: f32,
    pub y: f32,
}

impl MapPosition {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
