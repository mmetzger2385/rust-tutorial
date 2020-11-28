use std::io;
use rand::Rng;

fn main() {
    let low = 1;
    let high = 10;

    println!("Guess the number!");

    println!("What's yer guess? ({}-{})", low, high);

    let secret_number = rand::thread_rng().gen_range(low, high+1);

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("You guessed: {}", guess);

    println!("The secret number was: {}", secret_number);
}
