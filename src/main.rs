mod asset_manager;
mod schedule;
mod state;
mod movement;
mod map;
mod grid;
mod tile;

mod prelude {
    pub use bevy::prelude::*;
    pub use bevy_hanabi::prelude::*;
    pub use crate::asset_manager::*;
    pub use crate::schedule::*;
    pub use crate::state::*;
    pub use crate::movement::*;
    pub use crate::map::*;
    pub use crate::grid::*;
    pub use crate::tile::*;
    //pub use leafwing_input_manager::prelude::*;
    //pub use rand::{thread_rng, Rng};
}

use bevy::window::WindowResolution;
use crate::prelude::*;

pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGHT: f32 = 800.0;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(
        WindowPlugin {
            primary_window: Some(Window {
                title: "QueueTycoon".to_string(),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                ..default()
            }),
            ..default()
        }
    ))
    .add_systems(Startup, (setup_camera, initialize_map))
    //.add_plugins(AssetManagerPlugin)  
    .add_plugins(HanabiPlugin)
    .add_plugins(StatePlugin)
    .add_plugins(SchedulePlugin)
    //.add_plugins(GridPlugin)
    .add_plugins(MovementPlugin)
    .run();
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn initialize_map(mut commands: Commands) {
    let map = Map::new(5, 5);
    let map_entity = commands.spawn((Map::new(map.width,map.height), Transform::default()))
    .id();
    show_grid(commands, &map, map_entity);
}