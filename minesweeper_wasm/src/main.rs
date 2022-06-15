mod lib;

use lib::MineSweeper;

fn main() {
    let mut ms = MineSweeper::new(10, 10, 16);

    ms.place_flag(3, 3);
    ms.click(4, 3);
    println!("{ms}");
}
