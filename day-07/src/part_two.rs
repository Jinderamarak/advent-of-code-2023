use std::collections::HashMap;

use super::parsing::{Hand, HandBid, HandTypes2};

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

const CARDS: &'static str = "J23456789TQKA";
const JOKER: char = 'J';

fn cards_to_strength(hand: &Hand) -> u64 {
    hand.cards
        .iter()
        .map(|c| CARDS.find(|x| *c == x).unwrap())
        .fold(0, |acc, c| acc * 16 + c as u64)
}

impl HandTypes2 for Hand {
    fn is_five_of_kind(&self) -> bool {
        for card in &self.cards {
            if self
                .cards
                .iter()
                .filter(|c| *c == card || **c == JOKER)
                .count()
                == 5
            {
                return true;
            }
        }

        false
    }

    fn is_four_of_kind(&self) -> bool {
        for card in &self.cards {
            if self
                .cards
                .iter()
                .filter(|c| *c == card || **c == JOKER)
                .count()
                == 4
            {
                return true;
            }
        }

        false
    }

    fn is_full_house(&self) -> bool {
        let jokers = self.cards.iter().filter(|c| **c == JOKER).count();
        for card1 in &self.cards {
            for card2 in &self.cards {
                if card1 != card2 && *card1 != JOKER && *card2 != JOKER {
                    let c1 = self.cards.iter().filter(|c| *c == card1).count();
                    let c2 = self.cards.iter().filter(|c| *c == card2).count();

                    if jokers == 2 && ((c1 == 1 && c2 == 2) || (c1 == 2 && c2 == 1)) {
                        return true;
                    }
                    if jokers == 1 && c1 == 2 && c2 == 2 {
                        return true;
                    }
                    if jokers == 0 && ((c1 == 3 && c2 == 2) || (c1 == 2 && c2 == 3)) {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn is_three_of_kind(&self) -> bool {
        for card in &self.cards {
            if self
                .cards
                .iter()
                .filter(|c| *c == card || **c == JOKER)
                .count()
                == 3
            {
                return true;
            }
        }

        false
    }

    fn is_two_pair(&self) -> bool {
        let jokers = self.cards.iter().filter(|c| **c == JOKER).count();
        for card1 in &self.cards {
            for card2 in &self.cards {
                if card1 != card2 && *card1 != JOKER && *card2 != JOKER {
                    let c1 = self.cards.iter().filter(|c| *c == card1).count();
                    let c2 = self.cards.iter().filter(|c| *c == card2).count();

                    if jokers == 2 && ((c1 == 0 && c2 >= 2) || (c1 >= 2 && c2 == 0)) {
                        return true;
                    }
                    if jokers == 1 && ((c1 >= 1 && c2 >= 2) || (c1 >= 2 && c1 >= 1)) {
                        return true;
                    }
                    if jokers == 0 && c1 >= 2 && c2 >= 2 {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn is_one_pair(&self) -> bool {
        let jokers = self.cards.iter().filter(|c| **c == JOKER).count();
        for card in &self.cards {
            let kind = self.cards.iter().filter(|c| *c == card).count();
            if kind == 2 || (*card != JOKER && kind + jokers >= 2) {
                return true;
            }
        }

        false
    }

    fn is_high_card(&self) -> bool {
        for card in &self.cards {
            let kind = self.cards.iter().filter(|c| *c == card).count();
            if kind > 1 {
                return false;
            }
        }

        true
    }
}
