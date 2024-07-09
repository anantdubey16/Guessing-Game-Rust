# Number Guessing Game in Rust

This is a simple number guessing game written in Rust. The program generates a random number between 1 and 100, and the player has to guess the number within a limited number of attempts.

## How to Create and Run the Guessing Game

Follow these steps to create and run the guessing game program:

### 1. Create a New Rust Project

Open your terminal and create a new Rust project using Cargo:

```sh
cargo new guessing_game
cd guessing_game
```
### 2. Install the rand crate 

```sh
cargo add rand
```
*Or*

### Add the rand crate to Cargo.toml

Open the Cargo.toml file and add the rand crate under [dependencies]:
```sh
[dependencies]
rand = "0.8.5"  # Check for the latest version on crates.io
```

### 3. Write the Guessing Game Program

Open the src/main.rs file in your preferred text editor & write the logic for guessing game.

### 4. Build and Run the Program

Go back to your terminal, build the project, and run the program:

```sh
cargo build
cargo run
```
### 5. Playing the Game

Once the program is running, you will see prompts to enter your guesses. Here is an example session:

![image](https://github.com/anantdubey16/Guessing-Game-Rust/assets/81023294/d491d9b8-054b-479b-95ff-516e7dc5b898)

*You can enter different numbers to guess the random number generated by the program. The program will provide feedback whether the guess is too low, too high, or correct.*
