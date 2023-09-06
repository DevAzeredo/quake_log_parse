use std::{
    collections::{HashMap, HashSet},
    env,
    fs::read_to_string,
    path::PathBuf,
};

use serde::Serialize;

use super::error::LogError;

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
/// Struct containing methods for working with log data.
pub struct LogModel {}
impl LogModel {
    /// Retrieves match data and player rankings from log content.
    ///
    /// This function reads the log content, processes the events, and returns
    /// match data and player rankings.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with the following meaning:
    ///
    /// - `Ok((matches, player_rank))` - Indicates that the operation was successful,
    ///   and it returns the extracted match data and player rankings.
    /// - `Err(LogError)` - Indicates that an error occurred while processing the log.
    ///   The `LogError` contains details about the encountered problem.
    pub fn get_matches_and_player_rank() -> Result<(Vec<Match>, Vec<PlayerScore>), LogError> {
        let mut matchs = Vec::new();
        let mut player_rank = Vec::new();
        self::process_events_matches(&mut matchs, &read_log()?)?;
        self::process_ranking(&mut matchs, &mut player_rank)?;
        Ok((matchs, player_rank))
    }
}
/// Processes events from the log file and updates the list of matches.
///
/// This function iterates through the lines of the log file content and updates the list of matches
/// based on the events found, such as game initialization, player kills, and client information changes.
/// If an error occurs during event processing, the function returns a `Result` with a `LogError`
/// indicating details about the error.
///
/// # Arguments
///
/// * `matches` - A mutable reference to a vector of `Match` representing the matches.
/// * `file_content` - A reference to the string containing the log file content.
///
/// # Returns
///
/// Returns a `Result` with the following semantics:
///
/// - `Ok(())` - Indicates that event processing was successful, and the matches were updated
///   successfully.
/// - `Err(LogError)` - Indicates that an error occurred during event processing. The `LogError`
///   contains details about the error, including an error description and the line where it occurred.

fn process_events_matches(matches: &mut Vec<Match>, file_content: &str) -> Result<(), LogError> {
    for line in file_content.lines() {
        match line {
            s if s.contains("InitGame:") => {
                if let Err(err) = process_init_game(matches) {
                    return Err(LogError::InitGameError(format!(
                        "Erro {:?} na linha {}",
                        err, s
                    )));
                }
            }
            s if s.contains("Kill:") => {
                let idx = matches.len();
                if let Err(err) = process_kill_line(s, &mut matches[idx - 1].data) {
                    return Err(LogError::KillError(format!(
                        "Erro {:?} na linha {}",
                        err, s
                    )));
                }
            }
            s if s.contains("ClientUserinfoChanged") => {
                let idx = matches.len();
                if let Err(err) = process_client_changed_line(s, &mut matches[idx - 1].data) {
                    return Err(LogError::ClientUserinfoChangedError(format!(
                        "Erro {:?} na linha {}",
                        err, s
                    )));
                }
            }
            _ => {}
        }
    }
    Ok(())
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
fn read_log() -> Result<String, LogError> {
    match read_to_string(get_log_path()?) {
        Ok(file_content) => Ok(file_content),
        Err(err) => Err(LogError::ReadLogError(format!(
            "Erro ao ler o arquivo de log: {}",
            err
        ))),
    }
}
/// Retrieves the path to the log file.
///
/// This function obtains the path to the log file named "qgames.log" located in the same directory as the executable.
/// If the log file does not exist, it returns a `LogError` with details about the error.
///
/// # Returns
///
/// Returns a `Result` with the following semantics:
///
/// - `Ok(PathBuf)` - Indicates that the log file path was successfully obtained, and it is returned as a `PathBuf`.
/// - `Err(LogError)` - Indicates that an error occurred while obtaining the log file path. The `LogError`
///   contains details about the error, including an error description.

fn get_log_path() -> Result<PathBuf, LogError> {
    let mut current_exe = match env::current_exe() {
        Ok(path) => path,
        Err(err) => {
            return Err(LogError::ExePathError(format!(
                "Erro ao obter o diretório do executavel, {}",
                err
            )))
        }
    };
    current_exe.pop();

    let mut path_log = current_exe.clone();
    path_log.push("qgames.log");

    match path_log.exists() {
        true =>  return Ok(path_log),
        false =>  Err(LogError::ReadLogError(format!("Erro ao obter o diretório do log, favor verifique se o arquivo qgames.log está presente no diretório: {}",current_exe.to_string_lossy().to_string()) )),
    }
}

fn process_init_game(games: &mut Vec<Match>) -> Result<(), LogError> {
    if games.len() >= i32::MAX as usize {
        return Err(LogError::InitGameError(
            "Número máximo de partidas alcançado.".to_string(),
        ));
    }
    games.push(Match {
        id: (games.len() + 1) as i32,
        data: MatchData::default(),
    });

    Ok(())
}
fn process_client_changed_line(line: &str, game: &mut MatchData) -> Result<(), LogError> {
    if let Some(inicio) = line.find("n\\") {
        if let Some(fim) = line.find("\\t\\") {
            let player_name = &line[inicio + 2..fim];
            game.players.insert(player_name.to_string());
        }
    }
    Ok(())
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
fn process_kill_line(line: &str, game: &mut MatchData) -> Result<(), LogError> {
    if line.contains("<world>") {
        parse_world_kill(line, game);
        insert_kills_by_means(line, game);
    } else {
        parse_player_kill(line, game);
        insert_kills_by_means(line, game);
    };
    game.total_kills += 1;
    Ok(())
}

pub fn process_ranking(
    matches: &mut Vec<Match>,
    ranking: &mut Vec<PlayerScore>,
) -> Result<(), LogError> {
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
    Ok(())
}
