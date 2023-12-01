use crate::AppState;
use bevy::prelude::*;

mod systems;
use systems::*;
mod components;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameOver), load_menu)
            .add_systems(OnExit(AppState::GameOver), tear_down_screen)
            .add_systems(
                Update,
                play_again_interaction.run_if(in_state(AppState::GameOver)),
            )
            .add_systems(
                Update,
                main_menu_interaction.run_if(in_state(AppState::GameOver)),
            );
    }
}
