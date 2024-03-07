use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    GameOn,
    Paused,
    GameOver,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
        .add_systems(Update,
            (
                game_state_input_events,
                transition_to_game.run_if(in_state(GameState::GameOver)),
            ),
        );
    }
}

fn game_state_input_events(
    mut next_state: ResMut<NextState<GameState>>,
    state: Res<State<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match state.get() {
            GameState::GameOn => next_state.set(GameState::Paused),
            GameState::Paused => next_state.set(GameState::GameOn),
            _ => (),
        }
        info!("Game State: {:?}", state);
    }
}

fn transition_to_game(mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::GameOn);
}