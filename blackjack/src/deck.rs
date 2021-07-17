use std::fmt;
use crate::card::Card;

use rand::seq::SliceRandom;
use rand::thread_rng;

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
        // shuffle before return deck object
        let mut rng = thread_rng();  
        cards.shuffle(&mut rng);
        Deck{cards: cards, idx: 0}
    }

    pub fn draw(&mut self) -> Card {
        let c = self.cards[self.idx];
        self.idx = (self.idx + 1) % 52;
        c.expect("None value found in Deck")
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = format!("Deck Object. {} undrawn cards.", 52 - self.idx);
        write!(f, "{}", name)
    }
}
