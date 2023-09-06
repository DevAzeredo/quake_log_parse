use std::io;

use quake_log_parse::controller::LogController;

mod quake_log_parse;

fn main() {
    execute_choice();
}

fn execute_choice() {
    loop {
        let mut choice = String::new();

        println!("Select an option:");
        println!("1. Report each match and a player ranking.");
        println!("2. Option B");
        println!("0. Exit");

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read the line");

        match choice.trim() {
            "1" => {
                println!("You chose Report each match and a player ranking.");
                if let Err(err) = LogController::report_match_and_player_rank() {
                    println!("Error: {:?}", err);
                }
            }
            "2" => {
                println!("You chose Option B");
                // Place your code for Option B here
                break;
            }
            "0" => {
                println!("Exiting the program.");
                return;
            }
            _ => {
                println!("Invalid option. Please try again.");
            }
        }
    }
}
