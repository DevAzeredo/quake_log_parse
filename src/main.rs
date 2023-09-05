use serde::Serialize;
use serde_json::json;
use std::{
    collections::{HashMap, HashSet},
    env,
    fs::read_to_string,
    io,
    path::PathBuf,
};

#[derive(Debug, Default, Serialize)]
struct GameData {
    total_kills: i32,
    players: HashSet<String>,
    kills: HashMap<String, i32>,
    kills_by_means: HashMap<String, i32>,
}

struct Game {
    name: String,
    data: GameData,
}

fn get_log_path() -> PathBuf {
    // Obtém o diretório do executável
    let mut current_exe = match env::current_exe() {
        Ok(path) => path,
        Err(err) => {
            panic!("Erro ao obter o diretório do executável: {:?}", err);
        }
    };

    // Remove o nome do executável para obter o diretório
    current_exe.pop();

    let mut path_log = current_exe.clone();
    path_log.push("qgames.log");

    match path_log.exists() {
        true =>  return path_log,
        false =>  panic!("Erro ao obter o diretório do log, favor verifique se o arquivo qgames.log está presente no diretório: {}",current_exe.to_string_lossy().to_string()),
    }
}

fn main() {
    let file_content = match read_to_string(get_log_path()) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Erro ao ler o arquivo: {:?}", err);
            return;
        }
    };

    let mut games: Vec<Game> = Vec::new();
    let mut current_game: Option<usize> = None;
    for line in file_content.lines() {
        if line.contains("InitGame:") {
            // Inicia um novo jogo
            let game_name = format!("game_{}", games.len() + 1);
            current_game = Some(games.len()); // Use Some para representar um jogo ativo
            games.push(Game {
                name: game_name,
                data: GameData::default(),
            });
        } else if let Some(current_game_idx) = current_game {
            if line.contains("Kill:") {
                parse_kill_line(line, &mut games[current_game_idx].data);
            }

            if line.contains("ClientUserinfoChanged") {
                parse_client_changed_line(line, &mut games[current_game_idx].data);
            }
        }
    }

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
            report_match_and_player_rank(games);
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
fn parse_client_changed_line(line: &str, game: &mut GameData) {
    if let Some(inicio) = line.find("n\\") {
        if let Some(fim) = line.find("\\t\\") {
            let player_name = &line[inicio + 2..fim];
            game.players.insert(player_name.to_string());
        }
    }
}
fn parse_world_kill(line: &str, game: &mut GameData) {
    if let Some(inicio) = line.find("killed") {
        if let Some(fim) = line.find("by") {
            let victim = &line[inicio + 7..fim - 1];
            game.kills
                .entry(victim.to_string())
                .and_modify(|e| *e -= 1)
                .or_insert(-1);
        }
    }
}

fn find_third_colon_occurrence(input: &str) -> Option<usize> {
    let mut colon_count = 0;
    for (index, char) in input.char_indices() {
        if char == ':' {
            colon_count += 1;
            if colon_count == 3 {
                return Some(index);
            }
        }
    }
    None
}

fn parse_player_kill(line: &str, game: &mut GameData) {
    if let Some(inicio) = find_third_colon_occurrence(line) {
        if let Some(fim) = line.find("killed") {
            let killer = &line[inicio + 2..fim - 1];
            game.kills
                .entry(killer.to_string())
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
    }
}
fn insert_kills_by_means(line: &str, game: &mut GameData) {
    let mean = match line.split_whitespace().last() {
        Some(last_word) => last_word,
        None => "NOT_DETECTED",
    };
    game.kills_by_means
        .entry(mean.to_string())
        .and_modify(|e| *e += 1)
        .or_insert(1);
}

fn parse_kill_line(line: &str, game: &mut GameData) {
    if line.contains("<world>") {
        parse_world_kill(line, game);
        insert_kills_by_means(line, game);
    } else {
        parse_player_kill(line, game);
        insert_kills_by_means(line, game);
    };
    game.total_kills += 1;
}

fn report_match_and_player_rank(games: Vec<Game>) {
    let mut player_ranking: Vec<(String, i32)> = Vec::new();
    let mut player_set: HashSet<&str> = HashSet::new();

    // Processa os jogos e atualiza o total de kills de cada jogador
    for game in games.iter() {
        for (player, kills) in &game.data.kills {
            if player_set.contains(player.as_str()) {
                // O jogador já está no ranking, atualize sua pontuação
                if let Some(player_entry) = player_ranking
                    .iter_mut()
                    .find(|(name, _)| *name == player.as_str())
                {
                    player_entry.1 += kills;
                }
            } else {
                // O jogador não está no ranking, adicione-o
                player_ranking.push((player.clone(), *kills));
                player_set.insert(player.as_str());
            }
        }

        println!(
            "{}",
            json!({
                game.name.clone(): {
                    "total_kills": game.data.total_kills,
                    "players": game.data.players,
                    "kills": game.data.kills
                }
            })
        );
    }

    // Ordena o Vec em ordem decrescente de kills
    player_ranking.sort_by(|a, b| b.1.cmp(&a.1));

    let player_ranking_json: Vec<_> = player_ranking
        .iter()
        .map(|(name, total_kills)| json!({ name: total_kills }))
        .collect();

    let json_player_ranking = json!({ "Player Ranking": player_ranking_json });
    println!("{}", json_player_ranking);
}
