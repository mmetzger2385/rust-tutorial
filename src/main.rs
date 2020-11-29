use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    //constants
    const LOW: u32 = 1;
    const HIGH: u32 = 10;

    //mutable variable
    let mut count = 0;

    println!("Guess the number!");

    //Variables are immutable by default
    //gen_range - the lower bound is inclusive. Upper bound is exclusive so add 1
    let secret_number = rand::thread_rng().gen_range(LOW, HIGH+1);

    loop {

        //use mut to make variables mutable
        let mut guess = String::new();

        println!("What's yer guess? ({}-{})", LOW, HIGH);

        io::stdin().read_line(&mut guess).expect("Failed to read line");

        //shadowing a variable, essentially creates a new variable
        //can be a different type
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

    //---tuples--//
    let guess_count = (2, 5, 8);

    //destructuring
    let (low, medium, high) = guess_count;

    //direct reference
    let low_guess = guess_count.0;
    let medium_guess = guess_count.1;
    let high_guess = guess_count.2;

}
