use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let low = 1;
    let high = 10;
    let mut count = 0;

    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(low, high+1);

    loop {
        let mut guess = String::new();

        println!("What's yer guess? ({}-{})", low, high);

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("That's not even a number!");
                continue;
            }
        };

        count = count + 1;

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Nailed it!");
                break;
            }
        }
    }

    println!("It took you {} guesses!", count);
}
