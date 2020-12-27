use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    //constants
    const LOW: u32 = 1;
    const HIGH: u32 = 10;

    println!("Guess the number!");

    game_loop(LOW,HIGH);

    //---arrays---//
    //arrays cannot change size
    let guess_count_arr: [i32; 3] = [2, 5, 8];

    //---tuples--//
    //just copying an array into a tuple, for no good reason.
    let guess_count = (guess_count_arr[0], guess_count_arr[1], guess_count_arr[2]);

    //destructuring
    let (low, medium, high) = guess_count;

    //direct reference
    let low_guess = guess_count.0;
    let medium_guess = guess_count.1;
    let high_guess = guess_count.2;
}

fn generate_secret_number(low:u32, high:u32) -> u32{
    //gen_range - the lower bound is inclusive. Upper bound is exclusive so add 1
    rand::thread_rng().gen_range(low, high+1)
}

fn game_loop(low: u32, high: u32){

    //Variables are immutable by default
    let secret_number = generate_secret_number(low,high);

    //mutable variable
    let mut count = 0;

    loop {

        //use mut to make variables mutable
        let mut guess = String::new();

        println!("What's yer guess? ({}-{})", low, high);

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

    interpret_results(count);

    println!("It took you {} guesses!", count);

}

fn interpret_results(count: i32){

    //---arrays---//
    //arrays cannot change size
    let guess_count_arr: [i32; 3] = [2, 5, 8];

    //---tuples--//
    //just copying an array into a tuple, for no good reason.
    let guess_count = (guess_count_arr[0], guess_count_arr[1], guess_count_arr[2]);

    //destructuring
    let (low, medium, high) = guess_count;

    //direct reference
    let low_guess = guess_count.0;
    let medium_guess = guess_count.1;
    let high_guess = guess_count.2;

}