extern crate rand;
use crate::deck::Deck;
use crate::user::User;
use std::io;

pub struct BlackJackSim {
    players: Vec<User>,
    pub deck: Deck,
    dealer: User,
    rounds: i32
}

impl BlackJackSim {
    fn new(rounds:i32) -> BlackJackSim {
        let mut players: Vec<User> = Vec::new();
        for i in 0..4 {
            players.push(User::new(i));
        }
        let d: Deck = Deck::new();
        BlackJackSim{players: players, deck: d, dealer: User::new(-1), rounds: rounds}
    }

    pub fn start(&mut self) {
        for _ in 0..self.rounds {
            self.get_bets();
            self.deal_hands();
            self.show_dealer_hand(false);
            for id in 0..self.players.len() {
                if self.players[id].bet > 0 {
                    self.player_turn(id);
                }
            }
            self.dealer_turn();
            self.show_dealer_hand(true);
            self.determine_winners();
            // self.payouts();
            self.show_results();
        }
        
    }

    fn get_bets(&mut self) {
        println!("getting bets");
        let mut i = 1;
        for player in self.players.iter_mut() {
            loop {
                let max = player.money;
                let message = format!("Player {}, Place a bet between {} and {}. Enter 0 to fold.", i, 5 , 100);
                let bet = get_int_input(message);
                match bet {
                    0 => {
                        player.bet = 0;
                        player.status = -1;
                        break;
                    }, b if b > max => {
                        println!("Too Hi.");
                    }, b if b < 5 => {
                        println!("Too Low.");
                    }, _ => {
                        player.bet = bet;
                        break;
                    }
                }
            }
            i += 1;
        }
    }

    fn deal_hands(&mut self) {
        for _ in 0..2 { // two cards per player
            for player in self.players.iter_mut() {
                player.cards.add(self.deck.draw());
            }
            self.dealer.cards.add(self.deck.draw());
        }
        println!("Cards dealt to players and dealer.");
    }

    fn show_dealer_hand(&self, show_all: bool) {
        println!("\n+++++++++++++++++++++\nDEALER IS SHOWING...");
        if !show_all {
            for card_num in 0..self.dealer.cards.num_cards() {
                if card_num == 0 {
                    println!("> {}", self.dealer.cards.get_first_card());
                } else {
                    println!("> HIDDEN CARD\n+++++++++++++++++++++");
                }
            }
        } else {
            println!("Dealers Cards\n{}", self.dealer.cards);
            if self.dealer.cards.is_busted() {
                println!("/// DEALER BUSTED ///")
            }
        }
    }

    fn player_turn(&mut self, player_id: usize) {
        loop {
            self.show_player_cards(player_id);
            if self.players[player_id].cards.get_score() == -1 {
                self.players[player_id].status = -1;
                break;
            }
            let decision = get_int_input(String::from("Hit (1) or Hold (2)?"));
            match decision {
                1 => {
                    self.players[player_id].cards.add(self.deck.draw());
                }, 2 => {
                    break;
                }, _ => {
                    println!("Invalid response {}\n Try Again!!!", decision);
                }
            }
        }
        
    }

    fn show_player_cards(&self, player_id: usize) {
        println!("\n### Player's {} HAND ###\n{}", player_id+1, self.players[player_id].cards);
        if  self.players[player_id].cards.get_score() == -1 {
            println!("BUSTED!")
        }
    }

    fn dealer_turn(&mut self) {
        loop {
            if self.dealer.cards.is_busted() {
                self.dealer.status = -1;
                break;
            }
            let total = self.dealer.cards.get_score();

            if total <= 16 {
                // hit
                self.dealer.cards.add(self.deck.draw());
            } else if total == 17 && self.dealer.cards.has_ace() {
                // hit on soft 17 because there's an ace
                self.dealer.cards.add(self.deck.draw());
            } else if total <= 21 {
                // between 17 - 21, pass
                break;
            } else if total > -1 {
                self.dealer.status = -1;
                break;
            } else {
                println!("total: {}", total);
                panic!("Unknown matching in dealer_turn().");
            }
        }
    }

    fn determine_winners(&mut self) {
        if self.dealer.status == -1 {
            for player in self.players.iter_mut() {
                if player.status == 0 {
                    player.status = 1;
                }
            }
        } else {
            let dealer_total = self.dealer.cards.get_score();
            for player in self.players.iter_mut() {
                if player.cards.get_score() > dealer_total {
                    player.status = 1;
                } else {
                    player.status = -1;
                }
            }
        }
    }

    pub fn show_results(&mut self) {
        for player in self.players.iter_mut() {
            if player.status == 1 {
                println!("Player {} won {} chips!", player.id +1, player.bet);
                player.bet = player.bet * 2;
            } else if player.status == -1 {
                println!("Player {} lost.", player.id + 1);
            } else {
                println!("Player {} status was 0", player.id +1);
            }
        }
    }
}

pub fn get_int_input( message: String) -> isize {
    println!("{}", message);
    let mut var = String::new();
    io::stdin()
        .read_line(&mut var)
        .expect("Failed to read line");
        
        let var: isize = match var.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("you messed up idiot! Try Again!");
                get_int_input(message)
            },
        };
    var
}