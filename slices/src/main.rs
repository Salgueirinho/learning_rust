fn main() 
{
	let s = String::from("Hello world!");
	let first_word = first_word_s(&s);
	println!("first_word = {}", first_word);
	
	{
		let str = String::from("Hello World");
		let hello = &str[0..5];
		let world = &str[6..11];
		println!("{},{}", hello, world);
	}
	{
		let str = String::from("Hello World");
		let hello = &str[..5];
		let world = &str[6..];
		println!("{},{}", hello, world);
	}
	{
		let mut str = String::from("Hello World");
		let fw = better_first_word(&str);
		println!("fw = {}", fw);
		str.clear();
	}
	{
		let a = [1, 2, 3, 4, 5];
		let b = &a[0..3];
		println!("b[2] = {}", b[2]);
	}

	return;
}

fn better_first_word(s: &String) -> &str
{
	let bytes = s.as_bytes();
	for (i, &item) in bytes.iter().enumerate()
	{
			if item == b' '
			{
				return &s[..i];
			}
	}
	&s[..]
}

fn first_word_s(s: &String) -> String 
{
	let pos = first_word(&s);
	String::from(&s[0..pos])
}

fn first_word(s: &String) -> usize
{
	let bytes = s.as_bytes();

	for (i, &item) in bytes.iter().enumerate()
	{
			if item == b' '
			{
				return i;
			}	
	}
	s.len()
}
