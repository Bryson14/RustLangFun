use std::fmt;
extern crate rand;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;

#[derive(Copy, Clone)]
pub struct Card {
    num: usize,
    suite: usize
}

impl Card {
    pub fn new(num: usize, suite: usize) -> Card {
        assert! (num >= 1 && num <= 13);
        assert! (suite <= 3); // 0=spades, 1=hearts, 2=clubs, 3=diamonds
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
        write!(f, "{}", name)
    }
}

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
        s.push_str(&format!("___ Score: {} ___\n", self.get_score()));
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

    pub fn num_cards(&self) -> usize {
        self.cards.len()
    }

    pub fn add(&mut self, card: Card) {
        if self.busted {
            panic!("Cannot accept another card because this hand alreaded busted!\n{}", self);
        }
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
        if score <= 10 && num_aces > 0 {
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

pub struct User {
    pub id: isize,
    pub status: isize, // 0 tie, -1 busted, 1 won
    pub bet: isize,
    pub money: isize,
    pub cards: Hand
}

impl User {
    pub fn new(id: isize) -> User {
        User{id: id, status: 0, bet: 0, money: 100, cards: Hand::new()}
    }
}

pub struct BlackJack {
    players: Vec<User>,
    pub deck: Deck,
    num_players: isize,
    dealer: User
}

impl BlackJack {
    pub fn new(num_players: isize) -> BlackJack {
        let mut players: Vec<User> = Vec::new();
        for i in 0..num_players {
            players.push(User::new(i));
        }
        let d: Deck = Deck::new();
        BlackJack{players: players, deck: d, num_players: num_players, dealer: User::new(-1)}
    }

    pub fn start(&mut self) {
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
        // self.payouts()
        self.show_results();          
        println!("Breaking");
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
            println!("Dealers Cards\n{}", self.dealer.cards)
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
                println!("DEALER BUSTED!");
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