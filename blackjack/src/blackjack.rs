use std::fmt;
extern crate rand;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Clone)]
pub struct Card {
    upper_val: usize,
    lower_val: usize,  // only relevant for ace
    num: usize,
    suite: usize
}

impl Card {
    pub fn new(u_val: usize, l_val: usize, num: usize, suite: usize) -> Card {
        assert! (num >= 1 && num <= 13);
        assert! (suite <= 3); // 0=spades, 1=hearts, 2=clubs, 3=diamonds
        Card{upper_val: u_val, lower_val: l_val, num: num, suite: suite}
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut name = String::new();
        if self.lower_val != self.upper_val && self.num != 1 {
            panic!("Ace is the only card with a different upper and lower value.");
        }
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