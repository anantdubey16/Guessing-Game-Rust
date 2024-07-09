use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100); // Generate random number between 1 and 100
    let allowed_attempts = 5; // Number of allowed guesses

    println!("Guess the number!");

    for attempt in 1..=allowed_attempts {
        println!("Attempt {} of {}", attempt, allowed_attempts);

        let mut guess = String::new();
        println!("Enter your guess (1-100):");

        // Read user input
        std::io::stdin().read_line(&mut guess)
            .expect("Failed to read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        if guess < 1 || guess > 100 {
            println!("Number must be between 1 and 100.");
            continue;
        }

        if guess == secret_number {
            println!("You guessed it! The number was {}", secret_number);
            break;
        } else if guess < secret_number {
            println!("Too low. Guess again!");
        } else {
            println!("Too high. Guess again!");
        }

        // Check for out-of-attempts inside the loop now
        if attempt == allowed_attempts {
            println!("You ran out of attempts. The number was {}", secret_number);
        }
    }
}
