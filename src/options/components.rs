use bevy::prelude::*;

#[derive(Component)]
pub struct Options;

#[derive(Component)]
pub struct PlayButton;

#[derive(Component)]
pub struct LoadButton;

#[derive(Component)]
pub struct NBundle;

#[derive(Component, Default, Debug)]
pub struct TextBox {
    pub active: bool,
    pub id: u32,
    pub text_style: TextStyle,
}

#[derive(Component)]
pub struct TextBoxTag {
    pub id: u32,
}

#[derive(Component)]
pub struct ErrorMessageTag;

#[derive(Resource, Clone)]
pub struct OptionsResult {
    pub money_per_player: usize,
    pub small_blind_amount: usize,
    pub big_blind_amount: usize,
    pub num_players: usize,
    pub is_loaded_game: bool,
}
