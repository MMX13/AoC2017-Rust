use std::io;

fn main() {
	let mut input = String::new();
	io::stdin().read_line(&mut input).unwrap();
	let input: Vec<u32> = input.trim().chars().map(|x| x.to_digit(10).unwrap()).collect();
	let output: u32 = input.iter().enumerate()
						.filter(|&x| *x.1==input[(x.0 + 1) % input.len()])
						.map(|x| *x.1)
						.sum();
	println!("{}", output);
}