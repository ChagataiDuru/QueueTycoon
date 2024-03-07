use crate::prelude::*;
use bevy::prelude::*;

pub const GRID_LINE_WIDTH: f32 = 2.0;

pub const TILE_COORDINATE_LABEL_FONT_COLOR: Color = Color::BLACK;
pub const TILE_COORDINATE_LABEL_FONT_SIZE: f32 = 32. * 0.25;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        //app.add_systems(Startup,show_grid);
    }
}

#[derive(Component)]
pub struct Grid;

fn spawn_grid_vertical_lines(commands: &mut Commands, map: &Map) {
    let line_length = map.height as f32 * 32.;
    for i in 0..=map.width {
        let position_anchor = MapPosition { x: i, y: 0 };
        info!("{:?}", position_anchor);
        let (line_x, _) = calculate_sprite_position(&position_anchor);
        info!("{:?}", line_x);
        commands.spawn((
            Grid,
            SpriteBundle {
                sprite: Sprite {
                    color: Color::BLUE,
                    custom_size: Some(Vec2::new(GRID_LINE_WIDTH, line_length)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    line_x - 32. / 2.0,
                    map.height as f32 * 32. / -2.,
                    0.5,
                ),
                ..default()
            },
        ));
    }
}

fn spawn_grid_horizontal_lines(commands: &mut Commands, map: &Map) {
    let line_length = map.width as f32 * 32.;
    for j in 0..=map.height {
        let position_anchor = MapPosition { x: 0, y: j };
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
                    map.width as f32 * 32. / 2.,
                    line_y + 32. - 32. / 2.,
                    0.5,
                ),
                ..default()
            },
        ));
    }
}

pub fn show_grid(mut commands: Commands, map: &Map, map_entity: Entity) {
    spawn_grid_vertical_lines(&mut commands, map);
    spawn_grid_horizontal_lines(&mut commands, map);
}

pub fn calculate_sprite_position(map_position: &MapPosition) -> (f32, f32) {
    (
        map_position.x as f32 * 32. + 32. / 2.0 , -1f32 * map_position.y as f32 * 32.- 32. / 2.0,
    )
}