use bevy::prelude::*;

use crate::AppState;

pub struct GameDoing;

impl Plugin for GameDoing{
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameRunning), setup_game).
        add_systems(Update,game_playing.run_if(in_state(AppState::LoopGameRuning)));
    }
}

// ten second update timer
const UPDATE_TIME_SECOND:f32=10f32;

#[derive(Resource)]
struct GameTimer(Timer);

/// setup game
fn setup_game(mut commands: Commands,mut state: ResMut<NextState<AppState>>){
    commands.insert_resource(GameTimer(Timer::from_seconds(UPDATE_TIME_SECOND, TimerMode::Repeating)));
    state.set(AppState::LoopGameRuning);
}

/// game_playing
fn game_playing(){

}
