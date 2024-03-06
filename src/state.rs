use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    InGame,
    Paused,
    GameOver,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>().add_systems(
            Update,
            (
                game_state_input_events,
                transition_to_in_game.run_if(in_state(GameState::GameOver)),
            ),
        );
    }
}