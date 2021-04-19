mod fibonacci;
use std::io;

fn main() {
	
	println!("Find the Fibonaccci Sequenc");
	
	let mut guess = String::new();

	io::stdin()
		.read_line(&mut guess)
		.expect("Failed to read line");

	let guess: u128 = guess.trim()
		.parse()
		.expect("Failed to parse user input into u32");
		
    println!("Fib {}", guess);
	println!("ans: {}", fibonacci::fib_dp(guess));
}
