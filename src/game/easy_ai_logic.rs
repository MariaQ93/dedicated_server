use super::cards::*;
use super::components::*;
use super::hand_evaluation::*;
use bevy::prelude::*;
use rand::Rng;
use std::collections::HashMap;

//Simply sets Ace to strongest card. All others remain the same
pub fn generate_card_strength(val: u8) -> u8 {
    if val == 1 {
        14
    } else {
        val
    }
}

pub fn generate_post_flop_hand_strength(
    hand: &mut Vec<Card>,
    community_query: &mut Query<&CommunityCards>,
) -> u16 {
    /*let mut hand_and_community = vec_hand.clone();
    for vector in &community.cards{
        hand_and_community.push(*vector);
    }*/

    let mut hand_and_community: Vec<Card> = Vec::new();
    hand_and_community.append(hand);

    for community_cards in community_query.iter() {
        hand_and_community.append(&mut community_cards.cards.to_vec())
    }

    //hand_and_community.append(&mut community_cards.cards);

    let best_hand = find_best_hand(&hand_and_community);
    best_hand.score as u16
}

//Generates hand strength from starting hand
pub fn generate_pre_flop_hand_strength(hand: &Vec<Card>) -> u16 {
    hand[0].card_strength as u16 + hand[1].card_strength as u16
}

//Checks rand number w/in ranges to determine move
pub fn generate_move(
    player: &mut Player,
    poker_turn: &ResMut<PokerTurn>,
    community_query: &mut Query<&CommunityCards>,
) -> String {
    //Check for poker phase
    let mut _num = 101;
    let mut chosen_dist = player.move_dist.get(&player.hand_strength);

    if poker_turn.phase == PokerPhase::PreFlop {
        chosen_dist = player.move_dist.get(&player.hand_strength);
        if player.big_blind && !poker_turn.pot_raised {
            _num = rand::thread_rng().gen_range(0..=100);
        } else {
            _num = rand::thread_rng().gen_range(chosen_dist.unwrap()[0]..=100);
        }
    } else if poker_turn.phase == PokerPhase::Flop
        || poker_turn.phase == PokerPhase::Turn
        || poker_turn.phase == PokerPhase::River
    {
        player.hand_strength =
            generate_post_flop_hand_strength(&mut player.cards, community_query);

        chosen_dist = player.move_dist.get(&(player.hand_strength + 30_u16));
        if !poker_turn.bet_made {
            _num = rand::thread_rng().gen_range(0..=100);
        } else {
            _num = rand::thread_rng().gen_range(chosen_dist.unwrap()[0]..=100);
        }
    } else {
        chosen_dist = player.move_dist.get(&(player.hand_strength + 30_u16));
        _num = rand::thread_rng().gen_range(0..=100);
    }

    if _num <= chosen_dist.unwrap()[0] {
        "Check".to_string()
    } else if _num <= chosen_dist.unwrap()[1] {
        "Fold".to_string()
    } else if _num <= chosen_dist.unwrap()[2] {
        "Call".to_string()
    } else {
        "Raise".to_string()
    }
}

//We have a vector of value ranges each representing a pre-flop move.
//Then fill up hashmap with key value pair
//Key: Hand strength, value: ranges
//Returns hashmap
pub fn fill_move_set() -> HashMap<u16, Vec<u16>> {
    let mut move_dist = HashMap::new();

    //Vector order: check, fold, call, raise
    let mut vec_of_dists: Vec<Vec<u16>> = vec![
        vec![35, 58, 85, 100], //4
        vec![34, 58, 85, 100], //5
        vec![34, 56, 84, 100], //6
        vec![34, 54, 83, 100],
        vec![33, 50, 82, 100], //8
        vec![33, 50, 82, 100],
        vec![33, 49, 82, 100], //10
        vec![33, 48, 82, 100],
        vec![31, 46, 82, 100], //12
        vec![30, 45, 81, 100],
        vec![29, 39, 79, 100], //14
        vec![29, 39, 79, 100],
        vec![28, 39, 79, 100], //16
        vec![27, 38, 79, 100],
        vec![27, 38, 78, 100], //18
        vec![25, 38, 78, 100],
        vec![23, 37, 77, 100], //20
        vec![23, 37, 77, 100],
        vec![21, 36, 78, 100], //22
        vec![20, 34, 77, 100],
        vec![18, 30, 75, 100], //24
        vec![18, 28, 75, 100],
        vec![18, 27, 74, 100], //26
        vec![17, 23, 70, 100],
        vec![14, 19, 69, 100], //28
        vec![33, 48, 82, 100], //30
        vec![31, 56, 73, 100], //31
        vec![24, 34, 64, 100], //32
        vec![24, 30, 63, 100], //33
        vec![23, 28, 62, 100], //34
        vec![19, 25, 60, 100], //35
        vec![19, 21, 60, 100], //36
        vec![18, 19, 50, 100], //37
    ];

    //Here we are filling out the postflop and after distributions
    let mut i = 8;
    while i > 0 {
        move_dist.insert((i - 1) + 30, vec_of_dists.pop().unwrap());

        i -= 1;
    }

    //Here we are filling out the preflop distributions
    let mut i: u16 = 28;
    while i > 3 {
        move_dist.insert(i, vec_of_dists.pop().unwrap());

        i -= 1;
    }

    move_dist
}
