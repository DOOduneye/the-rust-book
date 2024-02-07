use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("This is the secret number: {secret_number}");

    for _ in 0..5 {
        let guess = input();

        match guess.cmp(&guess) {
            Ordering::Less => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
       
    }
}

fn input() -> u32 {
    println!("> Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    let guess = guess.trim().parse::<u32>().expect("Please type a number!");

    println!("You guessed: {guess}");

    return guess
}

// Bad design as we can use match and ordering
fn check_guess(secret_number: i32, guess: i32) -> i8 {
    if guess == secret_number {
        println!("You win!");
        return 0;
    } else if guess < secret_number {
        println!("Too low!");
        return -1;
    } else {
        println!("Too high!");
        return 1;
    }
}


