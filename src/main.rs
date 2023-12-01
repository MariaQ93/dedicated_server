use bevy::{prelude::*, window::*};

pub mod screen;
mod options;
mod game;
mod game_over;
use game::GamePlugin;
use game_over::GameOverPlugin;
use options::OptionsPlugin;
use screen::{ScreenPlugin, ServerPlugin};
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    GameOptions,
    StartScreen, // The menu selection/create server button and any other user affordances
    ServerRunning, // Server is now active, right now has just an exit button
    GameRunning,   // Game Running
    LoopGameRuning, // In loop Game Running
    OneGameOver,    // One Game Over
    GameOver,   // Game over
    GameEnd, // if game is over or menu is close,go to this state
}

fn main()
{
    App::new()
        .add_state::<AppState>()
        .add_plugins((

            DefaultPlugins.set(WindowPlugin
            {
                primary_window: Some(Window
                {
                    present_mode: PresentMode::Fifo,
                    resolution: WindowResolution::new(1280.0, 720.0),
                    title: "School Sanctioned Gambling Server".to_string(),
                    ..default()
                }),
                ..default()
            }),
            OptionsPlugin,
            ScreenPlugin,
            ServerPlugin,
            GamePlugin,
            GameOverPlugin,
        ))
        .run();
}