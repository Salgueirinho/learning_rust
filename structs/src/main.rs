struct User
{
	username: String,
	name: String,
	email: String,
	active: bool,
}



fn main() 
{
	let u1 = User
	{
		username: String::from("delta"),
		name: String::from("Gonçalo Salgueirinho"),
		email: String::from("goncalosalgueirinho842@gmail.com"),
		active: true,
	};
	print_user(&u1);
	{
		let u2 = new_user(String::from("Joao Augusto"), String::from("something@bla.com"));
		print_user(&u2);
		let u3 = u2;
		print_user(&u3);
		let u4 = User
		{
			email: String::from("hehe@ex.com"),
			..u1
		};
		print_user(&u4);
	}
}


fn new_user(name: String, email: String) -> User
{
	User
	{
		username: String::from(""),
		name, //: name,
		email, //: email,
		active: true,
	}
}

fn print_user(user: &User)
{
	println!("username:\t {}\nname:\t\t {}\nemail:\t\t {}\nactive:\t\t {}", user.username, user.name, user.email, user.active);
}
