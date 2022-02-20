fn main() 
{
	let mut x = 100; 
	println!("Main function!");
	x = addfive(x);
	function(x, addfive(x));
	println!("x = {}", x);
	return;
}

fn function(x: i32, y: i32)
{
	println!("Other function with x = {} and x + 5 = {}", x, y);
}

fn addfive(value: i32) -> i32
{
	//let x = value + 5;
	//return x;
	value + 5
}

