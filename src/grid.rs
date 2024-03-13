use crate::{prelude::*, TOP_LEFT};
use bevy::prelude::*;

pub const GRID_LINE_WIDTH: f32 = 2.0;

pub const TILE_SIZE: f32 = 32.0;

//pub const TILE_COORDINATE_LABEL_FONT_COLOR: Color = Color::BLACK;
//pub const TILE_COORDINATE_LABEL_FONT_SIZE: f32 = 32. * 0.25;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        //app.add_systems(Startup,show_grid);
    }
}

#[derive(Component)]
pub struct Grid;

fn spawn_grid_vertical_lines(commands: &mut Commands, map: &Map) {
    let line_length = map.height as f32 * TILE_SIZE;
    for i in 0..=map.width {
        let position_anchor = MapPosition { x: i as f32, y: 0.0 };      
        let (line_x, _) = calculate_sprite_position(&position_anchor);
        commands.spawn((
            Grid,
            SpriteBundle {
                sprite: Sprite {
                    color: Color::BLUE,
                    custom_size: Some(Vec2::new(GRID_LINE_WIDTH, line_length)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    TOP_LEFT.x+(line_x - TILE_SIZE / 2.0),
                    TOP_LEFT.y+(map.height as f32 * TILE_SIZE / -2.0),
                    0.5,
                ),
                ..default()
            },
        ));
    }
}

fn spawn_grid_horizontal_lines(commands: &mut Commands, map: &Map) {
    let line_length = map.width as f32 * TILE_SIZE;
    for j in 0..=map.height {
        let position_anchor = MapPosition { x: 0.0, y: j as f32};
        let (_, line_y) = calculate_sprite_position(&position_anchor);
        commands.spawn((
            Grid,
            SpriteBundle {
                sprite: Sprite {
                    color: Color::CYAN,
                    custom_size: Some(Vec2::new(line_length, GRID_LINE_WIDTH)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    TOP_LEFT.x+(map.width as f32 * TILE_SIZE / 2.),
                    TOP_LEFT.y+(line_y + TILE_SIZE - TILE_SIZE / 2.),
                    0.5,
                ),
                ..default()
            },
        ));
    }
}

fn draw_wooden_tile(mut commands: Commands, map: &Map, position: MapPosition, tile: Res<TileAssets>) {
    let (x, y) = calculate_sprite_position(&position);
    commands.spawn((
        TileBundle {
            tile: Tile,
            r#type: TileType::Wood,
            position,
            sprite: SpriteBundle {
                texture: tile.woodtile.clone(),
                transform: Transform::from_xyz(TOP_LEFT.x + x, TOP_LEFT.y + y, 0.0),
                ..default()
            },
        },
    ));
}

pub fn show_grid(mut commands: Commands, map: &Map) {
    spawn_grid_vertical_lines(&mut commands, map);
    spawn_grid_horizontal_lines(&mut commands, map);
}

pub fn calculate_sprite_position(map_position: &MapPosition) -> (f32, f32) {
    (
        map_position.x as f32 * 32. + 32. / 2.0 , -1f32 * map_position.y as f32 * 32.- 32. / 2.0,
    )
}