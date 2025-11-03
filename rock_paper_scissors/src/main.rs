use std::io;
use rand::Rng;

const OPTIONS: [&str; 3] = ["rock", "paper", "scissors"];

fn main() {
    println!("Welcome to the game of Rock Paper Scissors!");

    let mut start_game = true;

    while start_game {
        let user_index = user_input();
        let computer_index = computer_input();
        produce_winner(user_index, computer_index);

        start_game = play_again();
    };
}

fn user_input() -> u8 {
    loop {
        println!("Please input your choice (rock, paper or scissors)");

        let mut input= String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Something went wrong! (function 'user_input' for debugging)");

        input = input.trim().to_lowercase();

        for (i, element) in OPTIONS.iter().enumerate() {
            if input == *element {
                println!("Your choice is '{}'!", *element);
                return i as u8
            }
        }
        println!("You did not input a valid choice. Please write one of the following: 'rock', 'paper' or 'scissors'.");
    }
}

fn computer_input() -> u8 {
    let index = rand::rng().random_range(0..OPTIONS.len());

    println!("My choice is '{}'!", OPTIONS[index]);

    return index as u8
}

fn produce_winner(user_index: u8, computer_index: u8) {
    if user_index == computer_index {
        println!("It's a draw!");
    } else if user_index == (computer_index + 2) % 3 {
        println!("You lose!");
    } else {
        println!("You win!");
    }
}

fn play_again() -> bool {
    println!("Would you like to play again (yes/no)?");

    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Something went wrong! (function 'play_again for debugging");

        input = input.trim().to_lowercase();

        if input == "yes" { 
            println!("One more round it is!");
            return true
        } else if input == "no" { 
            println!("Bye!");
            return false
        } else { 
            println!("You did not input a valid choice. Please write one of the following: 'yes' or 'no'.");
            continue
        }
     }
}