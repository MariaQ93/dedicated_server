use super::cards::*;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Component)]
pub struct Background;

#[derive(Component)]
pub struct DealButton;

#[derive(Component)]
pub struct CheckButton;

#[derive(Component)]
pub struct CallButton;

#[derive(Component)]
pub struct RaiseButton;

#[derive(Component)]
pub struct FoldButton;

#[derive(Component)]
pub struct NBundle;

#[derive(Component)]
pub struct NCards;

#[derive(Component)]
pub struct VisPlayerCards;

#[derive(Component)]
pub struct VisPlayers;

#[derive(Component)]
pub struct VisText;

#[derive(Component)]
pub struct Blind;

#[derive(Component, Serialize, Deserialize)]
pub struct Player {
    pub player_id: usize,
    pub cards: Vec<Card>,
    pub cash: usize,
    pub current_bet: usize,
    pub has_folded: bool,
    pub has_moved: bool,
    pub is_all_in: bool,
    pub has_raised: bool,
    pub hand_strength: u16,
    pub move_dist: HashMap<u16, Vec<u16>>,
    pub big_blind: bool,
    pub small_blind: bool,
    pub cfr_data: HashMap<usize, CfrData>,
}

// Data to store all cfr_data necessary for Hard AI
#[derive(Clone, Serialize, Deserialize)]
pub struct CfrData {
    pub strategy: HashMap<String, f64>,
    pub cumulative_strategy: HashMap<String, f64>,
    pub regret_sum: HashMap<String, f64>,
}

#[derive(Component, Serialize, Deserialize)]
pub struct CommunityCards {
    pub cards: Vec<Card>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub enum PokerPhase {
    PreFlop,
    Flop,
    Turn,
    River,
    Showdown,
}

#[derive(Resource)]
pub struct StartingCash {
    starting_cash: usize,
}

#[derive(Serialize, Deserialize)]
pub struct PokerTurn {
    pub current_player: usize,
    pub phase: PokerPhase,
    pub round_started: bool,
    pub pot: usize,
    pub current_top_bet: usize,
    pub pot_raised: bool,
    pub bet_made: bool,
    pub small_blind: usize,
    pub big_blind: usize,
    pub small_blind_val: usize,
    pub big_blind_val: usize,
    pub is_first_round: bool,
}
impl Resource for PokerTurn {}

#[derive(Component)]
pub struct AITimer {
    pub timer: Timer,
}

#[derive(Serialize, Deserialize)]
pub struct NumPlayers {
    pub player_count: usize,
}
impl Resource for NumPlayers {}

#[derive(Default, Debug)]
pub struct LastPlayerAction {
    pub action: Option<PlayerAction>,
}
impl Resource for LastPlayerAction {}

#[derive(Debug)]
pub enum PlayerAction {
    Raise,
    Check,
    Fold,
    Call,
    None,
}

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

#[derive(Resource)]
pub struct GameResult {
    pub id: usize,
}

#[derive(Component)]
pub struct SaveButton;
