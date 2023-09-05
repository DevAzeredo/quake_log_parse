use std::{
    collections::{HashMap, HashSet},
    env,
    fs::read_to_string,
    path::PathBuf,
};

use serde::Serialize;

#[derive(Debug, Default, Serialize)]
pub struct MatchData {
    pub total_kills: i32,
    pub players: HashSet<String>,
    pub kills: HashMap<String, i32>,
    pub kills_by_means: HashMap<String, i32>,
}
#[derive(Serialize)]
pub struct Match {
    pub id: i32,
    pub data: MatchData,
}
#[derive(Serialize)]
pub struct PlayerScore {
    pub name: String,
    pub kills: i32,
}

pub struct LogModel {}
impl LogModel {
    pub fn get_matches_and_player_rank() -> (Vec<Match>, Vec<PlayerScore>) {
        let mut matchs = Vec::new();
        let mut player_rank = Vec::new();
        self::process_events_matches(&mut matchs, read_log());
        self::process_ranking(&mut matchs, &mut player_rank);
        matchs.sort_by_key(|game| game.id);
        (matchs, player_rank)
    }
}
fn process_events_matches(matches: &mut Vec<Match>, file_content: String) {
    for line in file_content.lines() {
        match line {
            s if s.contains("InitGame:") => process_init_game(s, matches),
            s if s.contains("Kill:") => {
                let idx = matches.len();
                process_kill_line(s, &mut matches[idx - 1].data);
            }
            s if s.contains("ClientUserinfoChanged") => {
                let idx = matches.len();
                process_client_changed_line(s, &mut matches[idx - 1].data);
            }
            _ => {}
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
fn read_log() -> String {
    let file_content = match read_to_string(get_log_path()) {
        Ok(content) => content,
        Err(err) => {
            panic!("Erro ao ler o arquivo: {:?}", err);
        }
    };
    file_content
}
fn get_log_path() -> PathBuf {
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

fn process_init_game(line: &str, games: &mut Vec<Match>) {
    if line.contains("InitGame:") {
        games.push(Match {
            id: (games.len() + 1) as i32,
            data: MatchData::default(),
        });
    }
}
fn process_client_changed_line(line: &str, game: &mut MatchData) {
    if let Some(inicio) = line.find("n\\") {
        if let Some(fim) = line.find("\\t\\") {
            let player_name = &line[inicio + 2..fim];
            game.players.insert(player_name.to_string());
        }
    }
}
fn parse_world_kill(line: &str, game: &mut MatchData) {
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
fn parse_player_kill(line: &str, game: &mut MatchData) {
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

fn insert_kills_by_means(line: &str, game: &mut MatchData) {
    let mean = match line.split_whitespace().last() {
        Some(last_word) => last_word,
        None => "NOT_DETECTED",
    };
    game.kills_by_means
        .entry(mean.to_string())
        .and_modify(|e| *e += 1)
        .or_insert(1);
}
fn process_kill_line(line: &str, game: &mut MatchData) {
    if line.contains("<world>") {
        parse_world_kill(line, game);
        insert_kills_by_means(line, game);
    } else {
        parse_player_kill(line, game);
        insert_kills_by_means(line, game);
    };
    game.total_kills += 1;
}

pub fn process_ranking(matches: &mut Vec<Match>, ranking: &mut Vec<PlayerScore>) {
    let mut player_set: HashSet<&str> = HashSet::new();

    for mat in matches.iter() {
        for (player, kills) in &mat.data.kills {
            if player_set.contains(player.as_str()) {
                // O jogador já está no ranking, atualize sua pontuação
                if let Some(player_entry) = ranking.iter_mut().find(|entry| entry.name == *player) {
                    player_entry.kills += kills;
                }
            } else {
                // O jogador não está no ranking, adicione-o
                ranking.push(PlayerScore {
                    name: player.clone(),
                    kills: *kills,
                });
                player_set.insert(player.as_str());
            }
        }
    }

    // Ordena o Vec em ordem decrescente de kills
    ranking.sort_by(|a, b| b.kills.cmp(&a.kills));
}
