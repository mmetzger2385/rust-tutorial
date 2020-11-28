use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let low = 1;
    let high = 10;

    println!("Guess the number!");

    println!("What's yer guess? ({}-{})", low, high);

    let secret_number = rand::thread_rng().gen_range(low, high+1);

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please enter a number!");

    println!("You guessed: {}", guess);

    match  guess.cmp(&secret_number){
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("Nailed it!"),
    }

    println!("The secret number was: {}", secret_number);
}
