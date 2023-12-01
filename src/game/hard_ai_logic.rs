use super::components::*;

use rand::Rng;
use std::collections::HashMap;

// Different values for each hand, these are used as for each hand you will want to have a different probability for different actions
const HIGH_CARD: usize = 0;
const ONE_PAIR: usize = 1;
const TWO_PAIR: usize = 2;
const THREE_OF_A_KIND: usize = 3;
const STRAIGHT: usize = 4;
const FLUSH: usize = 5;
const FULL_HOUSE: usize = 6;
const FOUR_OF_A_KIND: usize = 7;
const STRAIGHT_FLUSH: usize = 8;
const ROYAL_FLUSH: usize = 9;

impl CfrData {
    pub fn new() -> CfrData {
        let actions = vec![
            "Fold".to_string(),
            "Call".to_string(),
            "Raise".to_string(),
            "Check".to_string(),
        ];

        let mut strategy = HashMap::new();
        let mut cumulative_strategy = HashMap::new();
        let mut regret_sum = HashMap::new();

        let initial_probability = 1.0 / actions.len() as f64;

        for action in actions {
            strategy.insert(action.clone(), initial_probability);
            cumulative_strategy.insert(action.clone(), 0.0);
            regret_sum.insert(action.clone(), 0.0);
        }

        CfrData {
            strategy,
            cumulative_strategy,
            regret_sum,
        }
    }
}

// Using the regret for each action determine the new probabilities for each action
pub fn update_strategy_for_hand(player: &mut Player, hand_category: usize) {
    if let Some(cfr_data) = player.cfr_data.get_mut(&hand_category) {
        let mut normalizing_sum = 0.0;
        for (_, regret) in cfr_data.regret_sum.iter() {
            let adjusted_regret = regret.max(0.0);
            normalizing_sum += adjusted_regret;
        }

        for (action, regret) in cfr_data.regret_sum.iter() {
            let strategy_value = if normalizing_sum > 0.0 {
                regret.max(0.0) / normalizing_sum
            } else {
                1.0 / cfr_data.regret_sum.len() as f64
            };
            cfr_data.strategy.insert(action.clone(), strategy_value);
        }

        for (action, strategy_value) in cfr_data.strategy.iter() {
            let cumulative_value = cfr_data
                .cumulative_strategy
                .entry(action.clone())
                .or_insert(0.0);
            *cumulative_value += strategy_value;
        }
    }
}

// Based on probability from regret return the action that was chosen
pub fn select_action_for_hand(player: &mut Player, hand_category: usize) -> String {
    if let Some(cfr_data) = player.cfr_data.get(&hand_category) {
        let mut rng = rand::thread_rng();
        let mut cumulative_probability = 0.0;
        let random_value = rng.gen::<f64>();

        for (action, probability) in &cfr_data.strategy {
            cumulative_probability += probability;
            if random_value <= cumulative_probability {
                return action.clone();
            }
        }

        cfr_data.strategy.keys().next().unwrap().clone()
    } else {
        "Fold".to_string()
    }
}

// After each decision was made determine what the utilities were for the other decisions and update the corresponding regret for that action
pub fn update_regrets_for_hand(
    player: &mut Player,
    hand_category: usize,
    actual_utility: f64,
    utilities: HashMap<String, f64>,
) {
    if let Some(cfr_data) = player.cfr_data.get_mut(&hand_category) {
        for (action, &counterfactual_utility) in utilities.iter() {
            let regret = counterfactual_utility - actual_utility;
            let current_regret = cfr_data.regret_sum.entry(action.clone()).or_insert(0.0);
            *current_regret += regret;
        }
    }
}
