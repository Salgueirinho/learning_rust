fn main() 
{
	//_testing();
	let s = String::from("hello");
	takes_ownership(s.clone()); // sends a clone that is dropped once out of scope. s still owns the initial String
	println!("<main> {}", s);

	let number = 12;
	makes_copy(number);
	println!("<main> number = {}", number);

	let str = gives_ownership();
	println!("<main> str = {}", str);
	
	let new_str = takes_and_gives_back(String::from("Give me back"));
	println!("<main> str = {}", new_str);

	let (str2, len) = calculate_length(new_str); // solutions: send a clone or send a reference
	println!("<main> {} has a size of {}", str2, len);

	return;
}

fn calculate_length(string: String) -> (String, usize)
{
	let len = string.len();
	(string, len)
}

fn gives_ownership() -> String
{
	let string = String::from("take this");
	string
}

fn takes_and_gives_back(string: String) -> String
{
	println!("<takes_and_gives_back> I had enough. {} is yours now", string);
	string
}

fn takes_ownership(str: String)
{
	println!("<takes_ownership> {}", str);
	return;
}

fn makes_copy(number : i32)
{
	println!("<makes_copy> number = {}", number);
	return;
}

fn _testing()
{
	{
		let s = String::from("Hello");
		println!("{}", s);

		let mut str = String::from("Hi");
		str.push_str(" There!");
		println!("{}", str);
	}
	{
		let x = 5;
		let y = x;
		println!("x = {} \ny = {}", x, y);
	}
	{
		let s1 = String::from("Hello");
		let mut s2 = s1.clone();
		s2.push_str(" World");
		println!("s1 = {}", s1);
		println!("s2 = {}", s2);
	}
	return;
}
