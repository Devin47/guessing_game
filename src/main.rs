use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number");

    // Name: gen_range
    // Description: Takes a range expessionas an argumentand generates a random number in the range.
    // Name: Loop
    //Description: Allows the program to let the user continue trying to guess the secret number even if they input the wrong number.
    let secret_number = rand::thread_rng().gen_range(1..101);
   loop {

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    // Name: continue
    // Description: Continues the game until the secret number is input.
    let guess: u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    println!("You guessed: {}", guess);
    // Name: break
    //Description: Stops the game once the secret number is input.
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;
        }
     }
   }
}