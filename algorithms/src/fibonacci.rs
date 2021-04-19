pub fn fib(i: u128) -> u128 {
	match i {
		0 | 1 => i,
		_ => fib(i-1) + fib(i-2)
	}
}

pub fn fib_dp(i: u128) -> u128 {
	if i > 186 {
		panic!("Unsigned 128 Integer can't handle larger fib sequence");
	}
	
	let mut arr: Vec<u128> = vec![0, 1, 1, 2, 3];
	if i < 5 {
		arr[i as usize]
	} else {
		for n in 5..=i {
			let a = arr[(n-1) as usize];
			let b = arr[(n-2) as usize];
			println!("a({}): {} + b({}): {}", n-1, a, n-2, b);
			arr.push(a + b);
		}
		println!("\n{:?}", arr);
		arr[i as usize]
	}
}