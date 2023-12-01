use super::components::*;
use super::easy_ai_logic::*;
use super::hand_evaluation::*;
use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
// use super::hard_ai_logic::*;
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Deck {
    pub cards: Vec<Card>,
}

pub fn init_cards_resource() -> Deck {
    Deck {
        cards: init_cards(),
    }
}
impl Resource for Deck {}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Suit {
    Hearts,
    Diamonds,
    Spades,
    Clubs,
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Card {
    pub _card_id: u8, // unique card id: hearts 0-12, diamonds 13-25, spades 26-38, clubs 39-51 (also serves as the sprite sheet index)
    pub suit: Suit,
    pub value: u8,         // ace: 1, 2: 2, ..., 10: 10, jack: 11, queen: 12, king: 13
    pub card_strength: u8, //this is the value but it concerts the ace to be 14 instead of 1
}

#[derive(Resource, Clone)]
pub struct SpriteData {
    pub atlas_handle: Handle<TextureAtlas>,
}

impl Card {
    fn new(_card_id: u8, suit: Suit, value: u8) -> Card {
        Card {
            _card_id,
            suit,
            value,
            card_strength: generate_card_strength(value),
        }
    }

    pub fn copy(card: &Card) -> Card {
        
        Card::new(card._card_id, card.suit, card.value)
    }
}
impl ToString for Card{
    fn to_string(&self)->String{
        let card_value = if self.value < 11 && self.value > 1 {
            
            self.value.to_string()
        } else if self.value == 11 {
            "Jack".to_string()
        } else if self.value == 12 {
            "Queen".to_string()
        } else if self.value == 13 {
            "King".to_string()
        } else {
            "Ace".to_string()
        };

        format!(
            "{value} of {suit}",
            value = { card_value },
            suit = {
                if self.suit == Suit::Hearts {
                    "Hearts"
                } else if self.suit == Suit::Diamonds {
                    "Diamonds"
                } else if self.suit == Suit::Spades {
                    "Spades"
                } else {
                    "Clubs"
                }
            }
        )
    }
}

pub fn init_cards() -> Vec<Card> {
    let mut cards: Vec<Card> = Vec::with_capacity(52);
    let mut total: u8 = 0;
    let suits: Vec<Suit> = vec![Suit::Hearts, Suit::Diamonds, Suit::Spades, Suit::Clubs];
    for suit in suits {
        for value in 1..14 {
            cards.push(Card::new(total, suit, value));
            total += 1;
        }
    }
    cards
}

pub fn shuffle_cards(cards: &mut Vec<Card>) {
    cards.shuffle(&mut thread_rng());
}

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
) {
    info!("Loading card assets!");
    let spritesheet_handle = asset_server.load("spritesheet2.png");
    let cards_atlas =
        TextureAtlas::from_grid(spritesheet_handle, Vec2::new(129., 230.), 53, 1, None, None);
    commands.insert_resource(SpriteData {
        atlas_handle: atlases.add(cards_atlas),
    });
}

pub fn deal_hands(player_count: usize, cards: &mut Vec<Card>, starting_cash: usize) -> Vec<Player> {
    let mut result: Vec<Player> = Vec::with_capacity(player_count);

    for player_id in 0..player_count {
        let mut cfr_data = HashMap::new();

        for hand_category in 0..=9 {
            cfr_data.insert(hand_category, CfrData::new());
        }

        let hand: Vec<Card> = cards.drain(0..2).collect();
        result.push(Player {
            player_id,
            cards: hand.clone(),
            cash: starting_cash,
            current_bet: 0,
            has_folded: false,
            has_moved: false,
            is_all_in: false,
            has_raised: false,
            hand_strength: generate_pre_flop_hand_strength(&hand),
            move_dist: fill_move_set(),
            big_blind: false,
            small_blind: false,
            cfr_data,
        });
    }
    result
}

#[allow(clippy::if_same_then_else)]
pub fn deal_com_function(
    cards: &mut Vec<Card>,
    community_query: &Query<&CommunityCards>,
) -> Vec<Vec<Card>> {
    let mut result: Vec<Vec<Card>> = Vec::with_capacity(5);
    // Dealing of Flop, Turn, and River
    let temp=match community_query.iter().count(){
        0=>{
            // return flop
            cards.drain(0..3).collect::<Vec<Card>>()
        }
        3..=4=>{
            // return turn,river
            cards.drain(0..1).collect::<Vec<Card>>()
        }
        _=>{
            unimplemented!()
        }
    };
    result.push(temp);
    result
}

pub fn card_function(community_query: &Query<&CommunityCards>, players: &[&Player]) -> usize {
    // Takes all cards from communtiy_query and flattens it to a single card vector for use
    let community_cards: Vec<Card> = community_query
        .iter()
        .flat_map(|cards| &cards.cards)
        .cloned()
        .collect();
    let mut hand1: Hand = Hand::_new_blank();
    let mut hand2: Hand = Hand::_new_blank();
    // Iterate through each player
    for player_cards_component in players.iter() {
        let player_cards = &player_cards_component.cards;
        // Ensure there are at least 5 cards between the player and community cards before evaluation
        if player_cards.len() + community_cards.len() >= 5 {
            let hand = test_evaluator(
                player_cards_component.player_id,
                player_cards.to_vec(),
                community_cards.to_vec(),
            );
            if player_cards_component.player_id == 0 {
                hand1 = hand;
            } else {
                hand2 = hand;
            }
        }
    }

    let comparison = compare_hands(&mut hand1, &mut hand2);
    if comparison == 1 {
        0
    } else if comparison == 2 {
        1
    } else {
        2
    }
}

pub fn spawn_player_cards(
    commands: &mut Commands,
    players: &Vec<Player>,
    query: &mut Query<(Entity, &mut Player)>,
    sprite_data: &Res<SpriteData>,
) {
    // If players don't exist create the entity, if they do just update their cards they hold
    for player in players {
        let mut player_exists = false;
        for (_entity, mut existing_player) in query.iter_mut() {
            if player.player_id == existing_player.player_id {
                existing_player.cards = player.cards.clone();
                existing_player.move_dist = player.move_dist.clone();
                existing_player.hand_strength = player.hand_strength;
                player_exists = true;
                break;
            } else {
                continue;
            }
        }
        if !player_exists {
            commands.spawn(Player {
                player_id: player.player_id,
                cards: player.cards.clone(),
                cash: player.cash,
                current_bet: player.current_bet,
                has_folded: player.has_folded,
                has_moved: player.has_moved,
                is_all_in: player.is_all_in,
                has_raised: player.has_raised,
                hand_strength: generate_pre_flop_hand_strength(&player.cards),
                move_dist: fill_move_set(),
                big_blind: false,
                small_blind: false,
                cfr_data: player.cfr_data.clone(),
            });
        }

        // Only ever show the cards of player 0 i.e. the human player to the screen
        if player.player_id == 0 {
            for (index, card) in player.cards.iter().enumerate() {
                let transform_x = 740. - (2. * 129. + 100.) + (index as f32) * (129. + 40.);
                let transform_y = -360. + 230. / 2. + 20.;
                commands
                    .spawn(SpriteSheetBundle {
                        sprite: TextureAtlasSprite {
                            index: card._card_id as usize,
                            // custom_size: Some(Vec2::new(58., 93.)),
                            ..default()
                        },
                        texture_atlas: sprite_data.atlas_handle.clone(),
                        // transform: Transform::from_xyz(left_shift, 317., 2.),
                        transform: Transform::from_xyz(transform_x, transform_y, 2.),
                        ..default()
                    })
                    .insert(VisPlayerCards);
            }
        }
    }
}

pub fn spawn_community_cards(
    commands: &mut Commands,
    com_cards: Vec<Vec<Card>>,
    community_query: &Query<&CommunityCards>,
    sprite_data: &Res<SpriteData>,
) {
    for cards in com_cards {
        for (index, card) in cards.iter().enumerate() {
            let left_shift = -3. * 81.
                + ((((community_query.iter().count() as f32) + 1.) * ((index as f32) + 1.)) * 81.);
            commands
                .spawn(SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        index: card._card_id as usize,
                        custom_size: Some(Vec2::new(58., 103.)),
                        ..default()
                    },
                    texture_atlas: sprite_data.atlas_handle.clone(),
                    // transform: Transform::from_xyz(left_shift, 317., 2.),
                    transform: Transform::from_xyz(left_shift, 0., 2.),
                    ..default()
                })
                .insert(CommunityCards {
                    cards: vec![*card],
                });
        }
    }
}
