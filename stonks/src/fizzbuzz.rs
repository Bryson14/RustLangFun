pub fn fizzbuzz(n: usize) {
    println!("FIZZBUZZ from 1 to {}", n);
    (1..=n).for_each(|x| println!("{}", printer(x)));
}

fn printer(n: usize) -> String {
    if n % 5 == 0 && n % 3 == 0 {
        String::from("FizzBuzz")
    } else if n % 5 == 0 {
        String::from("Fizz")
    } else if n % 3 == 0 {
        String::from("Buzz")
    } else {
        n.to_string()
    }
}
