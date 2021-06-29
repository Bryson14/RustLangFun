mod deck;

fn main() {
    println!("Hello, world!");
    let mut d: deck::Deck = deck::Deck::new();
    let card: &deck::Card = d.draw();
    println!("{}; val {} lower {}", card, card.upper_val, card.lower_val);
}
