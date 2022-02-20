const THREE_HOURS_IN_SECONDS:u32 = 60 * 60 * 3;

fn main() {
    {
        let mut x = 10;

        println!("x = {}", x);

        x = 5;
        println!("x = {}", x);
    
        println!("Three hours in seconds = {}", THREE_HOURS_IN_SECONDS);
    }
    {
        let y = 10;
        {
            let y = y * 2;
            println!("y = {}", y);
        }
        println!("y = {}", y);
    }
}
