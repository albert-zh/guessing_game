use rand::Rng;
/**  Guess.rs is used to defind a guess number generator.
 1. Receive a range from user;
 2. Generate a number in the range;
 3. Accept the guess number
 4. Return result
 5. Continue until the guess number is right
*/
use std::io;

/// targetNumber: generated number
/// guessNumber: a list of user's input
pub struct GuessGame {
    target_number: u64,
    guess_number: Vec<u64>,
}

impl GuessGame {
    /// start a game, initialize targetNumber
    pub fn new(range: (u64, u64)) -> GuessGame {
        return GuessGame {
            target_number: rand::thread_rng().gen_range(range.0, range.1),
            guess_number: Vec::new(),
        };
    }

    pub fn start(&mut self) -> bool {
        let mut is_match: bool = false;
        while !is_match {
            println!("Guess the number");

            println!("Please input your guess :");

            let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            let guess: u64 = guess.trim().parse().expect("Please type a number");

            self.guess_number.push(guess);
            match guess.cmp(&self.target_number) {
                std::cmp::Ordering::Equal => {
                    println!("You Win!");
                    is_match = true;
                }
                std::cmp::Ordering::Greater => println!("Too big"),
                std::cmp::Ordering::Less => println!("Too small"),
            }
        }
        true
    }
}
