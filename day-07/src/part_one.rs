use std::collections::HashMap;

use super::parsing::{Hand, HandBid, HandTypes1};

pub fn solve(data: &str) -> Vec<u32> {
    let mut hands = data
        .lines()
        .map(HandBid::parse)
        .map(|hb| (hand_to_strength(&hb.hand), hb))
        .collect::<Vec<_>>();

    hands.sort_by_key(|h| h.0);

    hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i + 1) as u32 * h.1.bid)
        .collect()
}

fn hand_to_strength(hand: &Hand) -> u64 {
    let cards = cards_to_strength(hand);
    let types = type_to_strength(hand);
    types * 16_u64.pow(hand.cards.len() as u32) + cards
}

fn type_to_strength(hand: &Hand) -> u64 {
    if hand.is_five_of_kind() {
        0x7
    } else if hand.is_four_of_kind() {
        0x6
    } else if hand.is_full_house() {
        0x5
    } else if hand.is_three_of_kind() {
        0x4
    } else if hand.is_two_pair() {
        0x3
    } else if hand.is_one_pair() {
        0x2
    } else if hand.is_high_card() {
        0x1
    } else {
        0x0
    }
}

const CARDS: &'static str = "23456789TJQKA";

fn cards_to_strength(hand: &Hand) -> u64 {
    hand.cards
        .iter()
        .map(|c| CARDS.find(|x| *c == x).unwrap())
        .fold(0, |acc, c| acc * 16 + c as u64)
}

impl Hand {
    fn group_cards(&self) -> Vec<u8> {
        let mut map: HashMap<char, u8> = HashMap::new();
        for card in &self.cards {
            map.entry(*card).and_modify(|c| *c += 1).or_insert(1);
        }

        map.into_values().collect()
    }
}

impl HandTypes1 for Hand {
    fn is_five_of_kind(&self) -> bool {
        self.group_cards().len() == 1
    }

    fn is_four_of_kind(&self) -> bool {
        self.group_cards().iter().any(|g| *g == 4)
    }

    fn is_full_house(&self) -> bool {
        self.group_cards().iter().all(|g| *g == 3 || *g == 2)
    }

    fn is_three_of_kind(&self) -> bool {
        let group = self.group_cards();
        group.iter().any(|g| *g == 3) && group.iter().all(|g| *g == 3 || *g == 1)
    }

    fn is_two_pair(&self) -> bool {
        self.group_cards().iter().filter(|g| **g == 2).count() == 2
    }

    fn is_one_pair(&self) -> bool {
        self.group_cards().iter().filter(|g| **g == 2).count() == 1
    }

    fn is_high_card(&self) -> bool {
        self.group_cards().iter().all(|g| *g == 1)
    }
}
