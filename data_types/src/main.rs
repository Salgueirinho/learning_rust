fn main() 
{		
	let x: i32 = -20;
	println!("x = {}", x);

	let y = 2.02;
	println!("y = {}", y);

	let z: f32 = 5.62;
	println!("z = {}", z);

	let sum = 5 + 12;
	println!("sum = {}", sum);

	let difference = 12 - 5;
	println!("difference = {}", difference);

	let product = 10 * 3;
	println!("product = {}", product);

	let quotient = 30 / 3;
	println!("division = {}", quotient);

	let remainder = 3 % 2;
	println!("remainder = {}", remainder);

	let mut active = true;
	println!("Active = {}", active);

	active = false;
	println!("Active = {}", active);
	
	let exists: bool;
	exists = false;
	println!("exists = {}", exists);
	
	let char_c = 'c';
	println!("char_c = {}", char_c);
	
	let cat = '😻';
	println!("cat = {}", cat);

	let mut tup: (i32, f64, u8) = (-17, 20.8, 255);
	let (x, y, z) = tup;
	println!("tup_x = {}", x);
	println!("tup_y = {}", y);
	println!("tup_z = {}", z);

	tup.0 = -64;
	println!("tup = ({},{},{})", tup.0, tup.1, tup.2);

	let mut array = [1, 2];
	println!("array = [{}, {}]", array[0], array[1]);

	array[1] = 200;
	println!("array = [{}, {}]", array[0], array[1]);

	let array: [i32; 5] = [1, 2, 3, 4, 5];
	println!("array = [{}, {}]", array[0], array[4]);

	let array = [100; 5];
	println!("array = [{}, {}]", array[0], array[4]);

	return;
}
