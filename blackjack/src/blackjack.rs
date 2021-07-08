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
        assert! (suite >= 0 && suite <= 3);

    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut name = String::new();
        match self.num {
            1 => name.add("Ace"),
            2..10 => name.add(self.num.to_string()),
            11 => name.add("Jack"),
            12 => name.add("Queen"),
            13 => name.add("King")

        }
        name.add(" of ");
        match self.suite {
            0 => name.add("Spades"),
            1 => name.add("Hearts"),
            2 => name.add("Clubs"),
            3 => name.add("Diamonds"),
        }
        write!(f, "Card: {}", name)
    }

}