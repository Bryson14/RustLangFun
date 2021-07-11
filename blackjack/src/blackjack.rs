use std::fmt;
extern crate rand;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Copy, Clone)]
pub struct Card {
    num: usize,
    suite: usize
}

impl Card {
    pub fn new(num: usize, suite: usize) -> Card {
        assert! (num >= 1 && num <= 13);
        assert! (suite <= 3); // 0=spades, 1=hearts, 2=clubs, 3=diamonds
        let lower = num;
        Card{ num: num, suite: suite}
    }

    pub fn get_value(&self) -> usize {
        match self.num {
            1 => return 11,
            10 | 11 | 12 | 13 => return 10,
            2 ..= 9 => return self.num,
            _ => panic!("Get Value got unexpected value for card number.")
        }
    }

    pub fn get_busted_value(&self) -> usize { // for checking if busted
        if self.num == 1 {
            1
        } else {
            self.get_value()
        }
    }

    pub fn is_ace(&self) -> bool {
        self.num == 1
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut name = String::new();
        match self.num {
            1 => name.push_str("Ace"),
            2..=10 => name.push_str(self.num.to_string().as_str()),
            11 => name.push_str("Jack"),
            12 => name.push_str("Queen"),
            13 => name.push_str("King"),
            _ => panic!("An unknown number was in self.num while printing the card!")

        }
        name.push_str(" of ");
        match self.suite {
            0 => name.push_str("Spades"),
            1 => name.push_str("Hearts"),
            2 => name.push_str("Clubs"),
            3 => name.push_str("Diamonds"),
            _ => panic!("An unknown number was in self.suite while printing the card!")
        }
        write!(f, "Card: {}", name)
    }
}

pub struct Deck {
    cards: [Option<Card>; 52],
    idx: usize,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards: [Option<Card>; 52] = [None; 52];
        let mut i = 0;
        for suite in 0..4 {
            for num in 1..14 {
                cards[i] = Some(Card::new(num, suite));
                i += 1;
            }
        }
        Deck{cards: cards, idx: 0}
    }

    pub fn draw(&mut self) -> Card {
        let c = self.cards[self.idx];
        self.idx = (self.idx + 1) % 52;
        c.expect("None value found in Deck")
    }
}