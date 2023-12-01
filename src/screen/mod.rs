pub mod resource;
pub mod sd;
//rustuomod game_in;
mod server;
mod start_screen;
mod wait_screen;
mod running_screen;

pub(crate) use super::AppState;
use bevy::prelude::*;
use bevy_egui::*;
pub(crate) use resource::{GameInteraction, GameSigned, Message, User, UserInfo, Users,UiInfo,UiInfoString,PlayerInfos};
use resource::reset_infostring;
pub use sd::S2D;
pub struct ScreenPlugin;
pub use server::ServerPlugin;
impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameInteraction>()
        .init_resource::<UiInfo>()
            .init_resource::<Users>()
            .add_plugins(EguiPlugin)
            .add_systems(Update, reset_infostring.run_if(in_state(AppState::StartScreen).or_else(in_state(AppState::ServerRunning))))
            .add_systems(
                Update,
                start_screen::start_screen_update.run_if(in_state(AppState::StartScreen)),
            )
            .add_systems(
                Update,
                wait_screen::wait_screen_update.run_if(in_state(AppState::ServerRunning)),
            );
    }
}
