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
        let position_anchor = MapPosition { x: TOP_LEFT.x + i as f32, y: 0.0 };
        /*         
        let (line_x, _) = calculate_sprite_position(&position_anchor);
        info!("line_x: {}", line_x);
        info!(
            "Spawning vertical line at x: {}, y: {}", line_x - TILE_SIZE / 2.0, map.height as f32 * TILE_SIZE / -2.0
        ); */
        commands.spawn((
            Grid,
            SpriteBundle {
                sprite: Sprite {
                    color: Color::BLUE,
                    custom_size: Some(Vec2::new(GRID_LINE_WIDTH, line_length)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    position_anchor.x + TILE_SIZE / 2.0,
                    position_anchor.y - map.height as f32 * TILE_SIZE / -2.0,
                    0.5,
                ),
                ..default()
            },
        ));
        info!(
            "Spawning vertical line at x: {}, y: {}", position_anchor.x - TILE_SIZE / 2.0,
            position_anchor.y - map.height as f32 * TILE_SIZE / -2.0
        );
    }
}

fn spawn_grid_horizontal_lines(commands: &mut Commands, map: &Map) {
    let line_length = map.width as f32 * TILE_SIZE;
    for j in 0..=map.height {
        let position_anchor = MapPosition { x: 0.0, y: TOP_LEFT.y - j as f32};
/*         let (_, line_y) = calculate_sprite_position(&position_anchor);
        info!("line_y: {}", line_y);
        info!(
            "Spawning horizontal line at x: {}, y: {}", map.width as f32 * TILE_SIZE / 2.0, line_y + TILE_SIZE - TILE_SIZE / 2.0
        ); */
        commands.spawn((
            Grid,
            SpriteBundle {
                sprite: Sprite {
                    color: Color::CYAN,
                    custom_size: Some(Vec2::new(line_length, GRID_LINE_WIDTH)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    position_anchor.x - map.width as f32 * TILE_SIZE / 2.,
                    position_anchor.y + TILE_SIZE - TILE_SIZE / 2.,
                    0.5,
                ),
                ..default()
            },
        ));
        info!(
            "Spawning horizontal line at x: {}, y: {}", position_anchor.x - map.width as f32 * TILE_SIZE / 2.,
            position_anchor.y + TILE_SIZE - TILE_SIZE / 2.
        );
    }
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