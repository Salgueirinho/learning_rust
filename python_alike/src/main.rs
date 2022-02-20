use std::io::{self, Write, BufRead};

fn main() 
{
	let inp = input("Input your name: ").unwrap();
	println!("Hi {}", inp);
	return;
}


fn input(prompt: &str) -> io::Result<String> 
{
    print!("{}", prompt);
    io::stdout().flush()?;
    io::stdin()
        .lock()
        .lines()
        .next()
        .unwrap()
        .map(|x| x.trim_end().to_owned())
}
