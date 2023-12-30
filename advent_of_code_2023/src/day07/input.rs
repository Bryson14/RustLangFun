use std::collections::HashMap;

/// Reads the input into a vector of CamelCardHands
pub fn read_input(input: &str, include_jokers: bool) -> Vec<CamelCardHand> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(" ").filter(|s| !s.is_empty());
            if split.clone().count() != 2 {
                panic!("Invalid input: {}", line);
            }
            let cards = split.next().unwrap();
            let bid = split.next().unwrap().parse::<u32>().unwrap();
            let cards = cards
                .chars()
                .map(|card| match card {
                    'A' => CamelCard::A,
                    'K' => CamelCard::K,
                    'Q' => CamelCard::Q,
                    'J' if !include_jokers => CamelCard::J,
                    'J' if include_jokers => CamelCard::Joker,
                    'T' => CamelCard::T,
                    '2'..='9' => CamelCard::N(card.to_digit(10).unwrap()),
                    _ => panic!("Invalid card: {}", card),
                })
                .collect();
            let power_type = find_power_type(&cards);

            if include_jokers {
                let og_cards = convert_jokers(&cards);
                CamelCardHand {
                    cards,
                    bid,
                    power_type,
                    og_cards: Some(og_cards),
                    include_jokers,
                }
            } else {
                CamelCardHand {
                    cards,
                    bid,
                    power_type,
                    og_cards: None,
                    include_jokers,
                }
            }
        })
        .collect()
}

/// Gets the power type of the hand
fn find_power_type(cards: &Vec<CamelCard>) -> PowerType {
    let mut card_counts = HashMap::new();
    for card in cards {
        let count = card_counts.entry(card).or_insert(0);
        *count += 1;
    }

    let mut counts = card_counts.values().collect::<Vec<_>>();
    counts.sort();
    counts.reverse();

    match counts.as_slice() {
        [5] => PowerType::FiveOfAKind,
        [4, 1] => PowerType::FourOfAKind,
        [3, 2] => PowerType::FullHouse,
        [3, 1, 1] => PowerType::ThreeOfAKind,
        [2, 2, 1] => PowerType::TwoPair,
        [2, 1, 1, 1] => PowerType::OnePair,
        _ => PowerType::HighCard,
    }
}

// turns Any Jokers into the best card possible
fn convert_jokers(cards: &Vec<CamelCard>) -> Vec<CamelCard> {
    todo!("convert jokers");
    if cards.contains(&CamelCard::Joker) {
        let mut cards = cards.clone();

        // find number of jokers
        let joker_count = cards
            .iter()
            .filter(|card| **card == CamelCard::Joker)
            .count();

        // if there are 5 jokers, then replace with 5 Aces
        // if there are 4 jokers, then replace with the other card
        // if there are 3 jokers, then replace with highest of the two cards
        // if there are 2 jokers, then replace with highest of the three cards or the two cards if they match
        // if there is 1 joker, check swapping with other cards to see if it improves the hand with find_power_type()
        if joker_count == 5 {
            return vec![CamelCard::A; 5];
        } else if joker_count == 4 {
            let other_card = cards
                .iter()
                .find(|card| **card != CamelCard::Joker)
                .unwrap();
            return vec![*other_card; 5];
        } else if joker_count == 3 {
            let mut other_cards = cards
                .iter()
                .filter(|card| **card != CamelCard::Joker)
                .collect::<Vec<_>>();
            let highest_card = other_cards.iter().max().unwrap();

            // must keep order of cards
            // get the position of the other_card[1]
            let mut hand: Vec<CamelCard> = vec![];
            for card in cards {
                match card {
                    CamelCard::Joker => hand.push(*highest_card.clone()),
                    _ => hand.push(card.clone()),
                }
            }
            return hand;
        } else if joker_count == 2 {
            // if the two cards are the same, then replace with 2 more of that card
            // collect cards into a set, and if the set has length of 3, then the two cards are the same
            let mut other_cards = cards
                .iter()
                .filter(|card| **card != CamelCard::Joker)
                .collect::<Vec<_>>();

            let other_cards_set = other_cards.iter().collect::<std::collections::HashSet<_>>();
            if other_cards_set.len() == 3 {
                // the two cards are the same
                let mut hand: Vec<CamelCard> = vec![];
                let highest_card = other_cards.iter().max().unwrap();
                todo!("replace with 2 more of the same card");
                // for card in cards.clone() {
                //     match card {
                //         CamelCard::Joker => hand.push(**highest_card.clone()),
                //         _ => hand.push(card.clone()),
                //     }
                // }
                return hand;
            } else {
                // the match jokers to the two matching cards
                todo!("match jokers to the two matching cards");
            }
            // otherwise, replace with the highest of the three cards
        } else if joker_count == 1 {
            // find the best card to swap with the joker
            // make a vec of all the possible hands swapping the joker with another card, then sort them by power type, then return the best one
            let mut best_cards = cards.clone();
            let mut best_power_type = find_power_type(&best_cards);
            for i in 0..5 {
                let mut new_cards = cards.clone();
                new_cards[i] = CamelCard::Joker;
                let power_type = find_power_type(&new_cards);
                if power_type > best_power_type {
                    best_cards = new_cards;
                    best_power_type = power_type;
                }
            }

            return best_cards;
        } else {
            panic!("Invalid number of jokers: {}", joker_count);
        }

        // find the best card possible to swap based on CamelCardHand sorting and power type
    } else {
        cards.clone()
    }
}

/// swaps the card in swap_idx with every other card and returns a list of all the possible hands
fn create_possible_hands(cards: &Vec<CamelCard>, swap_idx: usize) -> Vec<Vec<CamelCard>> {
    if cards[swap_idx] != CamelCard::Joker {
        panic!("Can only swap jokers");
    }

    // replace the joker with every other card
    // make 13 copies of the hand, one for each card
    let mut hands = vec![];
    for i in 0..13 {
        let mut hand = cards.clone();
        hand[swap_idx] = match i {
            0 => CamelCard::A,
            1 => CamelCard::K,
            2 => CamelCard::Q,
            // no jack
            4 => CamelCard::T,
            5 => CamelCard::N(9),
            6 => CamelCard::N(8),
            7 => CamelCard::N(7),
            8 => CamelCard::N(6),
            9 => CamelCard::N(5),
            10 => CamelCard::N(4),
            11 => CamelCard::N(3),
            12 => CamelCard::N(2),
            _ => panic!("Invalid card index: {}", i),
        };
        hands.push(hand);
    }
    hands
}

// represents a playing card
// A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2
#[derive(Debug, PartialEq, Eq, PartialOrd, Hash, Clone)]
pub enum CamelCard {
    A,
    K,
    Q,
    J, // jack or joker but making a phantom Joker so std::cmp::Ord can be implemented
    T,
    N(u32),
    Joker,
}

impl CamelCard {
    fn get_strength(&self) -> u32 {
        match self {
            CamelCard::A => 14,
            CamelCard::K => 13,
            CamelCard::Q => 12,
            CamelCard::J => 11,
            CamelCard::T => 10,
            CamelCard::N(value) => *value,
            CamelCard::Joker => 0,
        }
    }
}

impl Ord for CamelCard {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.get_strength().cmp(&other.get_strength())
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub enum PowerType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Ord for PowerType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (PowerType::FiveOfAKind, PowerType::FiveOfAKind) => std::cmp::Ordering::Equal,
            (PowerType::FiveOfAKind, _) => std::cmp::Ordering::Greater,
            (_, PowerType::FiveOfAKind) => std::cmp::Ordering::Less,

            (PowerType::FourOfAKind, PowerType::FourOfAKind) => std::cmp::Ordering::Equal,
            (PowerType::FourOfAKind, _) => std::cmp::Ordering::Greater,
            (_, PowerType::FourOfAKind) => std::cmp::Ordering::Less,

            (PowerType::FullHouse, PowerType::FullHouse) => std::cmp::Ordering::Equal,
            (PowerType::FullHouse, _) => std::cmp::Ordering::Greater,
            (_, PowerType::FullHouse) => std::cmp::Ordering::Less,

            (PowerType::ThreeOfAKind, PowerType::ThreeOfAKind) => std::cmp::Ordering::Equal,
            (PowerType::ThreeOfAKind, _) => std::cmp::Ordering::Greater,
            (_, PowerType::ThreeOfAKind) => std::cmp::Ordering::Less,

            (PowerType::TwoPair, PowerType::TwoPair) => std::cmp::Ordering::Equal,
            (PowerType::TwoPair, _) => std::cmp::Ordering::Greater,
            (_, PowerType::TwoPair) => std::cmp::Ordering::Less,

            (PowerType::OnePair, PowerType::OnePair) => std::cmp::Ordering::Equal,
            (PowerType::OnePair, _) => std::cmp::Ordering::Greater,
            (_, PowerType::OnePair) => std::cmp::Ordering::Less,

            (PowerType::HighCard, PowerType::HighCard) => std::cmp::Ordering::Equal,
            (PowerType::HighCard, _) => std::cmp::Ordering::Greater,
            (_, PowerType::HighCard) => std::cmp::Ordering::Less,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
pub struct CamelCardHand {
    pub cards: Vec<CamelCard>,
    pub og_cards: Option<Vec<CamelCard>>,
    pub bid: u32,
    pub power_type: PowerType,
    include_jokers: bool,
}

impl Ord for CamelCardHand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // compare their power types
        // if equal, then comare their cards starting with the first card until a difference is found
        match self.power_type.cmp(&other.power_type) {
            std::cmp::Ordering::Equal => {
                for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
                    match self_card.cmp(other_card) {
                        std::cmp::Ordering::Equal => continue,
                        ordering => return ordering,
                    }
                }
                std::cmp::Ordering::Equal
            }
            ordering => ordering,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_read_input() {
        let input = "32T39 893
        A9942 54";

        let expected = vec![
            CamelCardHand {
                cards: vec![
                    CamelCard::N(3),
                    CamelCard::N(2),
                    CamelCard::T,
                    CamelCard::N(3),
                    CamelCard::N(9),
                ],
                og_cards: None,
                bid: 893,
                power_type: PowerType::OnePair,
                include_jokers: false,
            },
            CamelCardHand {
                cards: vec![
                    CamelCard::A,
                    CamelCard::N(9),
                    CamelCard::N(9),
                    CamelCard::N(4),
                    CamelCard::N(2),
                ],
                og_cards: None,
                bid: 54,
                power_type: PowerType::OnePair,
                include_jokers: false,
            },
        ];

        assert_eq!(read_input(input, false), expected);
    }

    #[test]
    fn test_ranking_hands() {
        let input = "33332 1
        2AAAA 2
        77888 3
        77788 4";

        // assert they are both 4 of a kind

        let hands = read_input(input, false);
        assert_eq!(hands[0].power_type, PowerType::FourOfAKind);
        assert_eq!(hands[1].power_type, PowerType::FourOfAKind);
        assert_eq!(hands[2].power_type, PowerType::FullHouse);
        assert_eq!(hands[3].power_type, PowerType::FullHouse);

        assert!(hands[0] > hands[1]);
        assert!(hands[2] > hands[3]);
    }
}
