use std::io;

use quake_log_parse::controller::LogController;

mod quake_log_parse;

fn main() {
    execute_choice();
}

fn execute_choice() {
    let mut choice = String::new();

    println!("Selecione uma opção:");
    println!("1. Opção A");
    println!("2. Opção B");

    io::stdin()
        .read_line(&mut choice)
        .expect("Falha ao ler a linha");
    match choice.trim() {
        "1" => {
            println!("Você escolheu a Opção A");
            LogController::report_match_and_player_rank();
        }
        "2" => {
            println!("Você escolheu a Opção B");
            // Coloque aqui o código para a Opção B
        }
        _ => {
            println!("Opção inválida");
        }
    }
}
