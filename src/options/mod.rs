use crate::AppState;
use bevy::prelude::*;

mod systems;
use systems::*;
pub mod components;

pub struct OptionsPlugin;

impl Plugin for OptionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup,|mut command:Commands|{command.spawn(Camera2dBundle::default());}).
        add_systems(OnEnter(AppState::GameOptions), load_options)
            .add_systems(OnExit(AppState::GameOptions), tear_down_options)
            .add_systems(
                Update,
                play_button_interaction.run_if(in_state(AppState::GameOptions)),
            )
            .add_systems(Update, (make_scrolly,handle_keyboard,activate,load_button_interaction).run_if(in_state(AppState::GameOptions)));
    }
}
