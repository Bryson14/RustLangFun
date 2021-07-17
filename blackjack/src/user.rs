use crate::hand::Hand;

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