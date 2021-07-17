use std::fmt;
use crate::card::Card;

pub struct Hand {
    cards: Vec<Card>,
    best_score: usize,
    busted: bool,
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        s.push_str("____ Hand: _____\n");
        for card in self.cards.iter() {
            s.push_str(&format!("  > {}\n", card));
        }
        s.push_str(&format!("___ Score: {} ___\n", self.calc_score()));
        write!(f, "{}", s)
    }
}

impl Hand {
    pub fn new() -> Hand {
        Hand{cards: Vec::new(), best_score:  0, busted: false}
    }

    pub fn get_score(&self) -> isize {
        if self.best_score <= 21 {
            self.best_score as isize
        } else {
            -1
        }
    }

    pub fn get_first_card(&self) -> Card {
        // used when dealer's first card is shown but the other one is covered
        if self.num_cards() > 0 {
            self.cards[0]
        } else {
            panic!("Dont have a card to give you");
        }
    }

    pub fn is_busted(&self) -> bool {
        self.busted
    }

    pub fn num_cards(&self) -> usize {
        self.cards.len()
    }

    pub fn add(&mut self, card: Card) {
        self.cards.push(card);
        let mut min_total = 0;
        for card in self.cards.iter() {
            min_total += card.get_busted_value();
        }
        if min_total > 21 {
            self.busted = true;
        }
        self.best_score = self.calc_score();
    }

    fn calc_score(&self) -> usize {
        // the key here is to deal with the ACES. however, realize that only one ace (if there are multiple), can possibly be an 11
        // Must add all none ace cards, then if there is room for an 11, add that once, then all other aces will be 1
        let mut num_aces = 0;
        let mut score = 0;
        for card in self.cards.iter() {
            if card.is_ace() {
                num_aces += 1;
            }
            score += card.get_busted_value();
        }
        if score <= 11 && num_aces > 0 {
            score += 10;
        }
        score
    }

    pub fn has_ace(&self) -> bool {
        for card in self.cards.iter() {
            if card.is_ace() {
                return true
            }
        }
        false
    }
}
