use super::cards::*;
use std::collections::HashSet;

#[derive(Clone)]
pub struct Hand {
    cards: Vec<Card>, //a vector with the 5 cards in a hand
    ranks: Vec<u8>,   //a vector that holds the number of each rank that is in the hand
    suits: u8,        //how many suits are present in the hand
    pub score: u8, //the final score of the hand -> 8: Straight flush, 7: Four of a kind, 6: Full house, 5: flush, 4: straight, 3: three of a kind, 2: two pair, 1: pair, 0: high card
}

impl Hand {
    pub fn is_empty(&self)->bool{
        self.cards.is_empty()
    }
    pub fn _new_blank() -> Hand {
        Hand {
            cards: Vec::new(),
            ranks: vec![0; 13],
            suits: 0,
            score: 0,
        }
    }
    pub fn new(cards: Vec<Card>) -> Hand {
        let mut suits: HashSet<Suit> = HashSet::new();
        let mut ranks: Vec<u8> = vec![0; 13];
        let score: u8 = 0;
        for &card in &cards {
            suits.insert(card.suit);
            ranks[usize::from(card.value - 1)] += 1;
        }
        Hand {
            cards,
            ranks,
            suits: u8::try_from(suits.len()).unwrap(),
            score,
        }
    }

    fn to_string(hand: &Hand) -> String {
        if hand.cards.is_empty() {
            return String::from("No cards in hand");
        }

        let card1 = Card::to_string(&hand.cards[0]);
        let card2 = Card::to_string(&hand.cards[1]);
        let card3 = Card::to_string(&hand.cards[2]);
        let card4 = Card::to_string(&hand.cards[3]);
        let card5 = Card::to_string(&hand.cards[4]);

        format!(
            "{card1}, {card2}, {card3}, {card4}, and {card5} with a score of {score}",
            card1 = card1,
            card2 = card2,
            card3 = card3,
            card4 = card4,
            card5 = card5,
            score = hand.score
        )
    }
}

pub fn test_evaluator(
    player_id: usize,
    player_cards: Vec<Card>,
    community_cards: Vec<Card>,
) -> Hand {
    let cards: Vec<Card> = player_cards
        .into_iter()
        .chain(community_cards)
        .collect();
    //use println below to see players cards in terminal
    //println!("{}", cards.iter().map(|card| card.to_string()).collect::<Vec<_>>().join(", "));
    let hand = find_best_hand(&cards);
    println!("Player {}: {}", player_id, Hand::to_string(&hand));
    hand
}

pub fn find_best_hand(cards: &Vec<Card>) -> Hand {
    //there might be a library to get all combinations easily
    if cards.len() == 5 {
        let mut hand: Hand = Hand::new(cards.to_vec());
        evaluate_hand(&mut hand);
        return hand;
    }

    let combinations: Vec<Vec<Card>> = get_all_hands(cards);

    let mut best_hand_val = 0;
    let mut best_hand: Hand = Hand::_new_blank();

    for combination in combinations {
        let mut cur_hand = Hand::new(combination);
        //println!("Current hand: {:?}", Hand::to_string(&cur_hand));
        evaluate_hand(&mut cur_hand);
        if cur_hand.score > best_hand_val {
            best_hand_val = cur_hand.score;
            best_hand = cur_hand
        } else if cur_hand.score == best_hand_val {
            let res = compare_hands(&mut cur_hand, &mut best_hand);
            if res == 1 {
                best_hand_val = cur_hand.score;
                best_hand = cur_hand
            } else if res == 0 {
                best_hand = cur_hand
            }
        }
        //println!("Best hand: {:?}", Hand::to_string(&best_hand));
    }

    best_hand
}

fn get_all_hands(cards: &Vec<Card>) -> Vec<Vec<Card>> {
    let mut combinations: Vec<Vec<Card>> = Vec::new();

    for i in 0..cards.len() {
        for j in (i + 1)..cards.len() {
            for k in (j + 1)..cards.len() {
                for l in (k + 1)..cards.len() {
                    for m in (l + 1)..cards.len() {
                        let combination = vec![
                            Card::copy(&cards[i]),
                            Card::copy(&cards[j]),
                            Card::copy(&cards[k]),
                            Card::copy(&cards[l]),
                            Card::copy(&cards[m]),
                        ];
                        combinations.push(combination)
                    }
                }
            }
        }
    }
    combinations
}

pub fn evaluate_hand(hand: &mut Hand) {
    let is_flush = hand.suits == 1;
    let is_straight = is_straight(&hand.ranks);
    let mut is_four = false;
    let mut is_three = false;
    let mut is_two_pair = false;
    let mut is_pair = false;

    for i in 0..hand.ranks.len() {
        if hand.ranks[i] == 4 {
            is_four = true;
        } else if hand.ranks[i] == 3 {
            is_three = true;
        } else if hand.ranks[i] == 2 && is_pair {
            is_two_pair = true;
        } else if hand.ranks[i] == 2 {
            is_pair = true;
        }
    }

    if is_straight && is_flush {
        //straight flush
        hand.score = 8;
    } else if is_four {
        //four of a kind
        hand.score = 7;
    } else if is_three && is_pair {
        //full house
        hand.score = 6;
    } else if is_flush {
        //flush
        hand.score = 5;
    } else if is_straight {
        //straight
        hand.score = 4;
    } else if is_three {
        //three of a kind
        hand.score = 3;
    } else if is_two_pair {
        //two pair
        hand.score = 2;
    } else if is_pair {
        //pair
        hand.score = 1;
    } else {
        //high card
        hand.score = 0;
    }
}

fn is_straight(ranks: &Vec<u8>) -> bool {
    let mut min_rank: usize = 14;
    let mut max_rank: usize = 0;
    let ace = ranks[0];
    let mut unique_ranks: u8 = 0;

    for i in 0..ranks.len() {
        if ranks[i] > 0 {
            unique_ranks += 1;

            if i > max_rank {
                max_rank = i;
            }

            if i < min_rank {
                min_rank = i;
            }
        }
    }

    if (max_rank - min_rank) == 4 && unique_ranks == 5 {
        true
    } else if ace > 0 {
        (max_rank - min_rank) == 3 && unique_ranks == 5 && (max_rank == 12 || min_rank == 1)
    } else {
        false
    }
}

//returns 1 if hand1 > hand2, 2 if hand2 > hand1, and 0 if they are equal
pub fn compare_hands(hand1: &mut Hand, hand2: &mut Hand) -> u8 {
    let score1 = hand1.score;
    let score2 = hand2.score;

    if score1 > score2 {
        return 1;
    } else if score2 > score1 {
        return 2;
    }

    if score1 == 1 {
        //compare pairs
        let pair1 = find_next(&hand1.ranks, 0, 2);
        let pair2 = find_next(&hand2.ranks, 0, 2);

        let comparison = compare(pair1, pair2);

        if comparison != 0 {
            return comparison;
        }
    } else if score1 == 2 {
        //compare two-pair
        let pair1 = find_next(&hand1.ranks, 0, 2);
        let pair2 = find_next(&hand2.ranks, 0, 2);

        let comparison = compare(pair1, pair2);

        if comparison != 0 {
            return comparison;
        }

        let pair3 = find_next(&hand1.ranks, usize::from(pair1 + 1), 2);
        let pair4 = find_next(&hand2.ranks, usize::from(pair2 + 1), 2);

        let comparison = compare(pair3, pair4);

        if comparison != 0 {
            return comparison;
        }
    } else if score1 == 3 {
        //compare three of a kind
        let three1 = find_next(&hand1.ranks, 0, 3);
        let three2 = find_next(&hand2.ranks, 0, 3);

        let comparison = compare(three1, three2);

        if comparison != 0 {
            return comparison;
        }
    } else if score1 == 6 {
        //compare full house
        let three1 = find_next(&hand1.ranks, 0, 3);
        let three2 = find_next(&hand2.ranks, 0, 3);

        let comparison = compare(three1, three2);

        if comparison != 0 {
            return comparison;
        }

        let pair1 = find_next(&hand1.ranks, 0, 2);
        let pair2 = find_next(&hand2.ranks, 0, 2);

        let comparison = compare(pair1, pair2);

        return comparison;
    } else if score1 == 7 {
        //compare four of a kind
        let four1 = find_next(&hand1.ranks, 0, 4);
        let four2 = find_next(&hand2.ranks, 0, 4);

        let comparison = compare(four1, four2);

        if comparison != 0 {
            return comparison;
        }
    }

    let mut ranks1: Vec<usize> = Vec::new();
    let mut ranks2: Vec<usize> = Vec::new();

    for i in 0..hand1.ranks.len() {
        if hand1.ranks[i] > 0 {
            if i == 0 {
                ranks1.append(&mut vec![14]);
            } else {
                ranks1.append(&mut vec![i]);
            }
        }
        if hand2.ranks[i] > 0 {
            if i == 0 {
                ranks2.append(&mut vec![14]);
            } else {
                ranks2.append(&mut vec![i]);
            }
        }
    }

    ranks1.reverse();
    ranks2.reverse();
    for i in 0..ranks2.len() {
        if ranks1[i] > ranks2[i] {
            return 1;
        } else if ranks2[i] > ranks1[i] {
            return 2;
        }
    }

    0
}

fn compare(mut val1: u8, mut val2: u8) -> u8 {
    if val1 == 0 {
        val1 = 14;
    }
    if val2 == 0 {
        val2 = 14;
    }

    if val1 > val2 {
        1
    } else if val2 > val1 {
        return 2;
    } else {
        return 0;
    }
}

fn find_next(ranks: &Vec<u8>, start_index: usize, value: u8) -> u8 {
    for i in start_index..ranks.len() {
        if ranks[i] == value {
            return u8::try_from(i).unwrap();
        }
    }
    0
}
