use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("I just guessed something between 1 and 99");

    // CHEAT: Print the secret number
    let secret_number = rand::thread_rng().gen_range(1..99);
    println!("CHEAT: The secret number is {}", secret_number);

    loop {
        println!("What is it?");
        
        // the guess of the user
        let mut guess = String::new();

        // Read the line with error handling (I love that)
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to retrieve the guess");

        // Redeclare 'guess' as i32? What about the previous 'guess' string variable?
        // A: Apprently it just replace it after that, prevent us from doing "guess_num" or "guess_str"
        // let guess: i32 = guess.trim().parse().expect("You failed to provide a number :)");
        let guess: i32 = match guess.trim().parse() {

            // If the number was succesfully parsed, return it
            Ok(num) => num,

            // _ mark the variable as unused, same as C#
            Err(_) => continue
        };

        // Print the guess because why not
        println!("Your guess is '{}'", guess);

        // This is like a switch expression in C#
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }

        // The standard if statement everyone knows
        if guess.cmp(&secret_number) == Ordering::Equal {
            println!("It looks like you won!");
            break;
        } else {
            println!("You failed to guessed it!");
        }
    }
}
