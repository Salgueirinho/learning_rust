#[derive(Debug)]
struct Rectangle
{
	width: f32,
	height: f32,
}

fn main() 
{
	let a = Rectangle::rectangle(20.3, 10.0);
	if a.width() && a.height() //returns true if width and height are greater than 0.0
	{
		println!("Area of a = {}", a.area());
		println!("Peremeter of a = {}", a.perimeter());
		println!("a is = {:#?}", a);
	}
	let b = Rectangle::square(20.0);
	if b.width() && b.height() //returns true if width and height are greater than 0.0
	{
		println!("Area of b = {}", b.area());
		println!("Peremeter of b = {}", b.perimeter());
		println!("a is = {:#?}", b);
	}
	return;
}



impl Rectangle
{
	fn rectangle(width: f32, height: f32) -> Rectangle
	{
		Rectangle 
		{
			width,
			height,
		}
	}
	fn square(size: f32) -> Rectangle
	{
		Rectangle
		{
			width: size,
			height: size,
		}
	}
	fn width(&self) -> bool
	{
		self.width > 0.0
	}
	fn height(&self) -> bool
	{
		self.height> 0.0
	}
	fn area(&self) -> f32
	{
		self.width * self.height
	}
	fn perimeter(&self) -> f32
	{
		self.width * 2.0 + self.height * 2.0
	}
}
