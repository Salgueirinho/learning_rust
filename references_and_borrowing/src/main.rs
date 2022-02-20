fn main() 
{
	{
		let mut str = String::from("hello there. how u doin?");

		let len = calculate_length(&str);
		println!("The length of {} is {}", str, len);
		
		change(&mut str);
		println!("{}", str);
	}
	{
		let mut s = String::from("Hello");
		println!("s = {}", s);

		{
			let r2 = &mut s;
			r2.push_str(" there");
			println!("r2 = {}", r2);
		}

		let r1 = &mut s;
		r1.push_str("!");
		println!("r1 = {}", r1);
	}
	{
		let mut s = String::from("Hello");

		let r1 = &s;
		let r2 = &s;
		println!("r1 = {}", r1);
		println!("r2 = {}", r2);
		// r1 and r2 are not used anymore so its safe for r3.
		let r3 = &mut s;
		println!("r3 = {}", r3);
	}
	{
		let reference_to_something = no_dangle();
		println!("no dangle = {}", reference_to_something);
	}
	return;
}

fn no_dangle() -> String
{
	let s = String::from("Something");
	s
}


//fn _dangle() -> &String
//{
//	let s =  String::from("Nothing");
//	&s // s goes out of scope. solution: just return de fkin String LOL 
//}

fn calculate_length(string: &String) -> usize
{
    string.len()
}

fn change(string: &mut String)
{
    string.push_str(" doin fine ma boi?");
}
