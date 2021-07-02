mod deck;
use deck::Deck;
use deck::Card;
use deck::User;
use std::io;

fn main() {
    println!("Hello, world!");
    let mut d: Deck = Deck::new();
    let num_players: i32;
    num_players = get_int_input(String::from("How many people are playing?"));

    //TODO create players
    let mut players : Vec<User> = Vec::new();
    for _ in 0..num_players+1 { // last user is the dealer.
        players.push(User::new());
    }

    loop{
        players = get_bets(players);
        players = deal_hands(players, d);
        println!("Cards of player 1 {}, {}", players[0].cards[0], players[0].cards[1]);
        show_dealer_hand(&players[players.len()-1], false);
        for i in 0..num_players {
            if players[i as usize].bet > 0 {
                players = player_turn(i as usize, players);
            }
        }
        // self.dealerTurn()
        // self.determineWinners()
        // self.payouts()
        // self.showResults()
        if get_int_input(String::from("Play Again? Yes (1) / No (9)")) != 1 {
            println!("Thanks for playing!");
            break;
        }
            
        

        println!("Breaking");
        break;
    }
    // self.showResults()
}

fn get_int_input(message: String) -> i32 {
    println!("{}", message);
    let mut var = String::new();
    io::stdin()
        .read_line(&mut var)
        .expect("Failed to read line");
        
        let var: i32 = match var.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("you messed up idiot! Try Again!");
                get_int_input(message)
            },
        };

    var
}

fn get_bets(mut players: Vec<User>) -> Vec<User>{
    for i in 0..players.len()-1 { //do not get bet from dealer
        loop {
            let bet = get_int_input(format!("Player {}: Place your bet between ${} and ${} (Enter 0 to skip):", i+1, 5, players[i].money));

            match bet {
                0 => {
                    println!("Player {} Quits", i+1);
                    break;
                },
                b if b > players[i].money=> {
                    println!("Bad Answer, try again.");
                },
                b if b < 5 => {
                    println!("Bad Answer, try again.");
                },
                _ => {
                    players[i].bet = bet;
                    break;
                }
            }
        }
    }
    players
}

fn deal_hands(mut players: Vec<User>, mut deck: Deck) -> Vec<User> {
    for _ in 0..2 {
        for player in players.iter_mut() {
            player.cards.push(deck.draw());
        }
    }
    players
}

fn show_dealer_hand(dealer: &User, round_done: bool) {
    let asdf = 5.0;
}

fn show_player_hand() {

}

fn player_turn(player_idx: usize, mut players: Vec<User>) -> Vec<User> {
    loop{
        show_player_hand();
        if players[player_idx].status != -1 {
            let res = get_int_input(format!("Player {}: Hit (1) or Hold (2)? ", player_idx+1));
            if res == 1 {
                //hit
                // let card: Card = deck.draw();
                // players[player_idx].cards.push(card);
                // let bust_sum = 0;
                // for c in players[player_idx].cards.iter() {
                //     bust_sum += c.lower_val;
                // }
                // if bust_sum > 21 {
                //     players[player_idx].status = -1;
                // }
            } else if res == 2 {
                println!("Player {} Holds\n <<====>>", player_idx+1);
                break;
            } else {
                println!("Idiot, try to enter something useful!");
            }
        } else {
            println!("Player {} busted...", player_idx+1);
            break;
        }
    }
    players
}