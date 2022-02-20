use std::io;

fn main() 
{		
	//let condition = true;
	//let number = if condition { 5 } else { 6 };
	//println!("number = {}", number);
	//_looping_a_lot();
	//_loop_again();
	//_compare_to_3();
	//_result_of_loop();
	//_while_coutdown();
	//_loop_through_array();	
	_range_loop();
	return;
}

fn _range_loop()
{
	for number in (1..4).rev()
	{
		println!("number = {}", number);
	}
	return;
}

fn _loop_through_array()
{
	let array = [10, 20, 30, 40, 50];
	for element in array
	{
		println!("Element = {}", element);
	}
	return;
}

fn _while_coutdown()
{
	let mut n = 3;
	while n >= 0
	{
		println!("{}", n);
		n-=1;
	}
	return;
}

fn _result_of_loop()
{
	let mut counter = 0;
	let result = loop
	{
			counter += 1;
			if counter == 10
			{
				break counter * 2;
			}
	};
	println!("Result = {}", result);
	return;
}

fn _looping_a_lot()
{
	let mut count = 0;
	'countin_up: loop
	{
		loop
		{
			println!("count = {}", count);
			count = count + 1;
			println!("Hi!");
			if count == 2
			{
				break 'countin_up;
			}
		}
	}
	return;
}

fn _loop_again()
{
	loop
	{
		println!("Again!");
	}
}

fn _compare_to_3()
{
	let mut input = String::new();
	
	println!("Insert a value");
	io::stdin().read_line(&mut input).unwrap();

	let value: f64 = input.trim().parse().unwrap();

	if value > 3.0
	{
		println!("Your number is greater then 3.0");
	}
	else
	{
		println!("Your number is less then 3.0");
	}

	return;
}
