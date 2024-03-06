mod schedule;
mod state;
mod movement;

mod prelude {
    pub use bevy::prelude::*;
    pub use bevy_hanabi::prelude::*;
    pub use crate::schedule::*;
    pub use crate::state::*;
    pub use crate::movement::*;
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
    .add_plugins(HanabiPlugin)
    .add_plugins(StatePlugin)
    .add_plugins(SchedulePlugin)
    .add_plugins(MovementPlugin)
    .add_systems(Startup, setup_camera)
    .run();
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}