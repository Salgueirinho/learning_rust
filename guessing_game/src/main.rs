use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() 
{
    let mut guess: String;
    let mut guess_number: u32;   
    let number = rand::thread_rng().gen_range(1.. 101);
    println!("Welcome to guessing game!");

    loop
    {
        guess = String::new();
        println!("Guess the number");

        // get input from stdin
        io::stdin().read_line(&mut guess).expect("Failed to read line!");
        println!("Your guess was: {}", guess);

        guess_number = match guess.trim().parse()
        {
            Ok(num) => num,
            Err(_) => continue,
        };

        match number.cmp(&guess_number)
        {
            Ordering::Less => println!("Your guess is to big!"),
            Ordering::Greater => println!("Your guess is to small!"),
            Ordering::Equal => 
                    {
                        println!("Your guess is right! Congrats");
                        break;
                    }
        }
    }

    return; 
}
