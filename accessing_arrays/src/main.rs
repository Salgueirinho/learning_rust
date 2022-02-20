use std::io;

fn main() 
{
	let array = [1, 2, 3, 4, 5];
	let mut input = String::new();

	println!("Choose a position");
	io::stdin().read_line(&mut input).unwrap();

	let index: usize = input.trim().parse().unwrap();
	println!("The value at {} is {}", index, array[index]);

	return; 
}
