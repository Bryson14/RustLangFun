mod lib;

use lib::MineSweeper;

fn main() {
    let mut ms = MineSweeper::new(5, 5, 3);
    for x in 0..5 {
        for y in 0..5 {
            println!("Clicked {},{}. Keep playing? {}", x, y, ms.click(x, y));
            println!("board: {}", ms);
        }
    }
}
