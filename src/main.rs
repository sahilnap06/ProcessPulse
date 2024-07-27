use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // Generate a secret number
    let secret_number : u32 = rand::thread_rng().gen_range(1..=100);

    // Accept input from the user in a loop
    loop {
        println!("What do you guess? : ");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line!!");
        // Try parsing the guess into int
        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess is Too small!"),
            Ordering::Greater => println!("Your guess is Too BIG!"),
            Ordering::Equal => {
                println!("You WIN!");
                break;
            }
        }
    }
    
}