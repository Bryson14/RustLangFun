use std::fmt;
extern crate rand;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct Card {
    pub upper_val: u8,
    pub lower_val: u8,  // only relevant for ace
    pub name: String,
    id: u8
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Card: {}", self.name)
    }
}
pub struct Deck {
    cards : Vec<Card>,
    current_idx: usize
}

impl Deck {

    pub fn new () -> Deck {
        let numbers = ["A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "Jack", "Queen", "King"];
        let suites = ["Diamonds", "Hearts", "Spades", "Clubs"];
        let mut my_cards = Vec::with_capacity(52);

        let mut counter = 0;
        for num in numbers.iter() {
            for suite in suites.iter() {
                let up: u8;
                let low: u8;
                let mut new_name = String::new();
                
                match num {
                    &"A" => {
                        up = 11;
                        low = 1;
                        new_name.push_str("Ace of ");
                        new_name.push_str(suite);
                    },
                    &"King" | &"Queen" | &"Jack" => {
                        up = 10;
                        low = 10;
                        new_name.push_str(num);
                        new_name.push_str(" of ");
                        new_name.push_str(suite);
                    }, 
                    &"2" | &"3" | &"4" | &"5" | &"6" | &"7" | &"8" | &"9" | &"10"=> {
                        new_name.push_str(num);
                        new_name.push_str(" of ");
                        new_name.push_str(suite);
                        up = num.parse().unwrap();
                        low = num.parse().unwrap();
                    },
                    _ => {
                        println!("Something went wrong in the match!");
                        use std::process;
                        process::exit(0x0100);
                    }

                }
                my_cards.push(Card{upper_val: up, lower_val: low, name: new_name, id: counter});
                counter += 1;
            }
        }  
        let mut rng = thread_rng();  
        my_cards.shuffle(&mut rng);
        Deck {cards: my_cards, current_idx: 0}  
    }

    pub fn draw(&mut self) -> &Card {
        let c: &Card = &self.cards[self.current_idx];
        self.current_idx += 1;
        &c
    }
}