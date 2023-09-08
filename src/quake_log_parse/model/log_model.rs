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
#[derive(Debug, PartialEq)]
enum MeansOfDeath {
    ModUnknown,
    ModShotgun,
    ModGauntlet,
    ModMachinegun,
    ModGrenade,
    ModGrenadeSplash,
    ModRocket,
    ModRocketSplash,
    ModPlasma,
    ModPlasmaSplash,
    ModRailgun,
    ModLightning,
    ModBfg,
    ModBfgSplash,
    ModWater,
    ModSlime,
    ModLava,
    ModCrush,
    ModTelefrag,
    ModFalling,
    ModSuicide,
    ModTargetLaser,
    ModTriggerHurt,
    ModNail,
    ModChaingun,
    ModProximityMine,
    ModKamikaze,
    ModJuiced,
    ModGrapple,
}

impl MeansOfDeath {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "MOD_UNKNOWN" => Some(MeansOfDeath::ModUnknown),
            "MOD_SHOTGUN" => Some(MeansOfDeath::ModShotgun),
            "MOD_GAUNTLET" => Some(MeansOfDeath::ModGauntlet),
            "MOD_JUICED" => Some(MeansOfDeath::ModJuiced),
            "MOD_KAMIKAZE" => Some(MeansOfDeath::ModKamikaze),
            "MOD_PROXIMITY_MINE" => Some(MeansOfDeath::ModProximityMine),
            "MOD_CHAINGUN" => Some(MeansOfDeath::ModChaingun),
            "MOD_NAIL" => Some(MeansOfDeath::ModNail),
            "MOD_TRIGGER_HURT" => Some(MeansOfDeath::ModTriggerHurt),
            "MOD_TARGET_LASER" => Some(MeansOfDeath::ModTargetLaser),
            "MOD_SUICIDE" => Some(MeansOfDeath::ModSuicide),
            "MOD_FALLING" => Some(MeansOfDeath::ModFalling),
            "MOD_TELEFRAG" => Some(MeansOfDeath::ModTelefrag),
            "MOD_CRUSH" => Some(MeansOfDeath::ModCrush),
            "MOD_LAVA" => Some(MeansOfDeath::ModLava),
            "MOD_SLIME" => Some(MeansOfDeath::ModSlime),
            "MOD_WATER" => Some(MeansOfDeath::ModWater),
            "MOD_BFG_SPLASH" => Some(MeansOfDeath::ModBfgSplash),
            "MOD_BFG" => Some(MeansOfDeath::ModBfg),
            "MOD_LIGHTNING" => Some(MeansOfDeath::ModLightning),
            "MOD_RAILGUN" => Some(MeansOfDeath::ModRailgun),
            "MOD_PLASMA_SPLASH" => Some(MeansOfDeath::ModPlasmaSplash),
            "MOD_PLASMA" => Some(MeansOfDeath::ModPlasma),
            "MOD_ROCKET_SPLASH" => Some(MeansOfDeath::ModRocketSplash),
            "MOD_ROCKET" => Some(MeansOfDeath::ModRocket),
            "MOD_GRENADE_SPLASH" => Some(MeansOfDeath::ModGrenadeSplash),
            "MOD_GRENADE" => Some(MeansOfDeath::ModGrenade),
            "MOD_MACHINEGUN" => Some(MeansOfDeath::ModMachinegun),
            "MOD_GRAPPLE" => Some(MeansOfDeath::ModGrapple),
            _ => None,
        }
    }
}
/// Struct containing methods for working with log data.
pub struct LogModel {}
impl LogModel {
    /// Retrieves game match data and player rankings from the game log.
    ///
    /// This function reads the game log file, processes its contents, and extracts information about
    /// game matches and player rankings. It returns the collected data as a tuple containing a vector
    /// of `Match` structs representing the list of matches and a vector of `PlayerScore` structs
    /// representing player rankings.
    ///
    /// # Returns
    ///
    /// * `Result<(Vec<Match>, Vec<PlayerScore>), LogError>` - A `Result` indicating success (`Ok`) or
    ///   an error (`Err`) if a problem is encountered during log processing.
    ///
    /// # Errors
    ///
    /// Returns an error of type `LogError` if any of the following conditions are met:
    ///
    /// * An error occurs while reading the game log file (`read_log()`).
    ///
    /// * An error occurs during event processing (`process_events_matches`) if there are issues with
    ///   parsing and updating match data.
    pub fn process_log() -> Result<(Vec<Match>, Vec<PlayerScore>), LogError> {
        let mut matchs = Vec::new();
        let mut player_rank = Vec::new();
        self::process_events_matches(&mut matchs, &read_log()?)?;
        self::process_ranking(&mut matchs, &mut player_rank);
        Ok((matchs, player_rank))
    }
}

/// Processes events in a log file and updates the list of matches.
///
/// This function iterates through log file lines and handles game initialization, player kills,
/// and changes in player information, updating the list of matches accordingly.
///
/// # Arguments
///
/// * `matches` - A mutable reference to a vector of `Match` structs representing the list of matches
///              to be updated during event processing.
///
/// * `file_content` - A string containing the content of the log file to be processed for events.
///
/// # Returns
///
/// * `Result<(), LogError>` - A `Result` indicating success (`Ok`) or an error (`Err`) if a problem
///   is encountered during event processing.
///
/// # Errors
///
/// Returns an error of type `LogError` if any of the following conditions are met:
///
/// * An error occurs during game initialization (`process_init_game`).
/// * An error occurs during processing a kill line (`process_kill_line`).
/// * An error occurs during processing a client connection line (`process_client_changed_line`).
pub fn process_events_matches(matches: &mut Vec<Match>, file_content: &str) -> Result<(), LogError> {
    for line in file_content.lines() {
        match line {
            s if s.contains("InitGame:") => {
                if let Err(err) = process_init_game(matches) {
                    return Err(LogError::InitGameError(format!(
                        "Error {:?} on line  {}",
                        err, s
                    )));
                }
            }
            s if s.contains("Kill:") => {
                let idx = matches.len();
                if let Err(err) = process_kill_line(s, &mut matches[idx - 1].data) {
                    return Err(LogError::KillError(format!(
                        "Error {:?} on line  {}",
                        err, s
                    )));
                }
            }
            s if s.contains("ClientUserinfoChanged") => {
                let idx = matches.len();
                if let Err(err) = process_client_changed_line(s, &mut matches[idx - 1].data) {
                    return Err(LogError::ClientUserinfoChangedError(format!(
                        "Error {:?} on line  {}",
                        err, s
                    )));
                }
            }
            _ => {}
        }
    }
    Ok(())
}

/// Finds the index of the third occurrence of the colon (':') character in a string.
///
/// This function iterates through the characters of the input string and returns the index
/// of the third colon character found.
///
/// # Arguments
///
/// * `input` - A string in which to find the third colon occurrence.
///
/// # Returns
///
/// * `Option<usize>` - Some(index) if the third colon is found, where `index` is the index
///   of the third colon character. None is returned if the third colon is not found.
pub fn find_third_colon_occurrence(input: &str) -> Option<usize> {
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
/// Reads the content of the log file and returns it as a string.
///
/// This function reads the content of the log file named "qgames.log" located in the same
/// directory as the executable and returns it as a string.
///
/// # Returns
///
/// * `Result<String, LogError>` - A `Result` indicating success (`Ok`) with the log file content as a
///   string, or an error (`Err`) if a problem is encountered during file reading.
///
/// # Errors
///
/// Returns an error of type `LogError` if any of the following conditions are met:
///
/// * The log file does not exist or cannot be read.
pub fn read_log() -> Result<String, LogError> {
    match read_to_string(get_log_path()?) {
        Ok(file_content) => Ok(file_content),
        Err(err) => Err(LogError::ReadLogError(format!(
            "Error reading the log file: {}",
            err
        ))),
    }
}
/// Retrieves the path of the log file.
///
/// # Returns
///
/// * `Result<PathBuf, LogError>` - A `Result` indicating success (`Ok`) or an error (`Err`) if a problem
///   is encountered while obtaining the log file path.
///
/// # Errors
///
/// Returns an error of type `LogError` if any of the following conditions are met:
///
/// * The log file named "qgames.log" located in the same directory as the executable does not exist.
pub fn get_log_path() -> Result<PathBuf, LogError> {
    let mut current_exe = match env::current_exe() {
        Ok(path) => path,
        Err(err) => {
            return Err(LogError::ExePathError(format!(
                "Error while obtaining the executable directory: {}",
                err
            )))
        }
    };
    current_exe.pop();

    let mut path_log = current_exe.clone();
    path_log.push("qgames.log");

    match path_log.exists() {
        true =>  return Ok(path_log),
        false =>  Err(LogError::ReadLogError(format!("Error while retrieving the log directory, please check if the qgames.log file is present in the directory: {}",current_exe.to_string_lossy().to_string()) )),
    }
}
/// Processes the initialization of a new game and adds it to the list of matches.
///
/// # Arguments
///
/// * `games` - A mutable reference to a vector of `Match` structs representing the list of matches.
///
/// # Returns
///
/// * `Result<(), LogError>` - A `Result` indicating success (`Ok`) or an error (`Err`) if a problem
///   is encountered during game initialization.
///
/// # Errors
///
/// Returns an error of type `LogError` if any of the following conditions are met:
///
/// * The maximum number of matches (i32::MAX) has been reached, and a new game cannot be initialized.
pub fn process_init_game(games: &mut Vec<Match>) -> Result<(), LogError> {
    if games.len() >= i32::MAX as usize {
        return Err(LogError::InitGameError(
            "Maximum number of matches reached.".to_string(),
        ));
    }
    games.push(Match {
        id: (games.len() + 1) as i32,
        data: MatchData::default(),
    });

    Ok(())
}
/// Processes a line of the log to extract player names when a player joins a team.
///
/// # Arguments
///
/// * `line` - A string containing the log line to be parsed.
/// * `game` - A mutable reference to the `MatchData` struct representing the game state.
///
/// # Returns
///
/// * `Result<(), LogError>` - A `Result` indicating success (`Ok`) or an error (`Err`) if a problem
///   is encountered during parsing.
///
/// # Errors
///
/// Returns an error of type `LogError` if any of the following conditions are met:
///
/// * The line does not contain the expected format, e.g., "n\player_name\t\".
/// * The extracted player name is empty.
pub fn process_client_changed_line(line: &str, game: &mut MatchData) -> Result<(), LogError> {
    if let Some(inicio) = line.find("n\\") {
        if let Some(fim) = line.find("\\t\\") {
            let player_name = &line[inicio + 2..fim];

            if !player_name.is_empty() {
                game.players.insert(player_name.to_string());
            } else {
                return Err(LogError::EmptyPlayerName(format!(
                    "Empty player name in the client changed line."
                )));
            }
        }
    }

    Ok(())
}

/// Parses a line of the log to extract information about a world kill and updates the game data.
///
/// # Arguments
///
/// * `line` - A string containing the log line to be parsed.
/// * `game` - A mutable reference to the `MatchData` struct representing the game state.
///
/// # Returns
///
/// * `Result<(), LogError>` - A `Result` indicating success (`Ok`) or an error (`Err`) if a problem
///   is encountered during parsing.
///
/// # Errors
///
/// Returns an error of type `LogError` if any of the following conditions are met:
///
/// * The keyword 'killed' is not found in the kill line.
/// * The keyword 'by' is not found in the kill line.
/// * The victim in the kill line is not found in the game's kill data.
pub fn parse_world_kill(line: &str, game: &mut MatchData) -> Result<(), LogError> {
    if let Some(inicio) = line.find("killed") {
        if let Some(fim) = line.find("by") {
            let victim = &line[inicio + 7..fim - 1];
            game.kills
                .entry(victim.to_string())
                .and_modify(|e| *e -= 1)
                .or_insert(-1);
        } else {
            return Err(LogError::MissingKeyword(format!(
                "The keyword 'by' was not found in the kill line."
            )));
        }
    } else {
        return Err(LogError::MissingKeyword(format!(
            "The keyword 'killed' was not found in the kill line."
        )));
    }

    Ok(())
}

/// Parses a line of the log to update kill statistics for a player.
///
/// This function extracts the name of the player who made a kill from the log line and updates
/// the kill count for that player in the game data.
///
/// # Arguments
///
/// * `line` - A string containing the log line to be parsed.
/// * `game` - A mutable reference to the `MatchData` struct representing the game state.
///
/// # Returns
///
/// * `Result<(), LogError>` - A `Result` indicating success (`Ok`) or an error (`Err`) if a problem
///   is encountered during parsing.
///
/// # Errors
///
/// Returns an error of type `LogError` if any of the following conditions are met:
///
/// * The third colon (":") expected in the log line is not found.
/// * The keyword "killed" is not found in the log line.
pub fn parse_player_kill(line: &str, game: &mut MatchData) -> Result<(), LogError> {
    if let Some(inicio) = find_third_colon_occurrence(line) {
        if let Some(fim) = line.find("killed") {
            let killer = &line[inicio + 2..fim - 1];
            game.kills
                .entry(killer.to_string())
                .and_modify(|e| *e += 1)
                .or_insert(1);
        } else {
            return Err(LogError::MissingKeyword(format!(
                "The keyword 'killed' was not found in the kill line."
            )));
        }
    } else {
        return Err(LogError::MissingKeyword(format!(
            "The third colon was not found in the kill line."
        )));
    }
    Ok(())
}
/// Inserts or increments the count of kills by means in a game match.
///
/// This function takes a log line and extracts the "mean" (typically a weapon or cause) of a kill from
/// the last word of the line. It then attempts to insert this mean into the `kills_by_means` map within
/// the `MatchData` structure. If the mean already exists in the map, its count is incremented. If the mean
/// does not exist, it is inserted into the map with a count of 1.
///
/// # Arguments
///
/// * `line` - A string representing the log line containing information about a kill event.
///
/// * `game` - A mutable reference to a `MatchData` struct representing the data of the game match.
///
/// # Returns
///
/// * `Result<(), LogError>` - A `Result` indicating success (`Ok`) or an error (`Err`) if a problem
///   is encountered during the insertion.
///
/// # Errors
///
/// Returns an error of type `LogError` if unable to extract the means in the log line, indicating an issue with the line's formatting.
///
/// Returns an error of type `LogError` if the extracted word is not a valid 'mean'.
pub fn insert_kills_by_means(line: &str, game: &mut MatchData) -> Result<(), LogError> {
    let last_word = match line.split_whitespace().last() {
        Some(last_word) => last_word,
        None => {
            return Err(LogError::InsertKillMeanError(format!(
                "Unable to extract the mean from line: {}",
                line
            )))
        }
    };

    match MeansOfDeath::from_str(last_word) {
        Some(_) => {
            game.kills_by_means
                .entry(last_word.to_string())
                .and_modify(|e| *e += 1)
                .or_insert(1);
        }
        None => {
            return Err(LogError::InsertKillMeanError(format!(
                r#"Mean '{}' not recognized as a valid means of death: {}"#,
                last_word, line
            )))
        }
    }

    Ok(())
}
/// Process a kill line from the game log.
///
/// This function is responsible for processing a line from the game log that represents a kill event.
/// It determines whether the kill event is a player kill or a kill by the world (e.g., environmental damage).
/// Depending on the type of kill event, it calls the corresponding parsing function (`parse_world_kill` or
/// `parse_player_kill`) to extract relevant information about the kill. It also updates the kills statistics
/// in the `MatchData` struct and records the means of death for later analysis.
///
/// # Arguments
///
/// * `line` - A string representing a line from the game log that contains a kill event.
///
/// * `game` - A mutable reference to a `MatchData` struct where kill statistics and means of death information
///   will be updated.
///
/// # Returns
///
/// * `Result<(), LogError>` - A `Result` indicating success (`Ok`) or an error (`Err`) if any problem
///   occurs during kill event processing.
///
/// # Errors
///
/// Returns an error of type `LogError` if there are any issues with processing the kill event or updating
/// the game data.
pub fn process_kill_line(line: &str, game: &mut MatchData) -> Result<(), LogError> {
    match line {
        _ if line.contains("<world>") => {
            if let Err(err) = parse_world_kill(line, game) {
                return Err(err);
            }
        }
        _ => {
            if let Err(err) = parse_player_kill(line, game) {
                return Err(err);
            }
        }
    };

    if let Err(err) = insert_kills_by_means(line, game) {
        return Err(err);
    }

    game.total_kills += 1;

    Ok(())
}

/// Process player rankings based on game match data.
///
/// This function calculates and updates player rankings based on the provided game match data. It takes
/// a mutable reference to a vector of `Match` structs and a mutable reference to a vector of `PlayerScore`
/// structs. Player rankings are updated with the total number of kills achieved by each player across all
/// matches. If a player is already in the ranking, their score is updated; otherwise, a new entry is added
/// to the ranking.
///
/// # Arguments
///
/// * `matches` - A mutable reference to a vector of `Match` structs containing game match data.
///
/// * `ranking` - A mutable reference to a vector of `PlayerScore` structs representing player rankings.
pub fn process_ranking(matches: &mut Vec<Match>, ranking: &mut Vec<PlayerScore>) {
    let mut player_set: HashSet<&str> = HashSet::new();

    for mat in matches.iter() {
        for (player, kills) in &mat.data.kills {
            if player_set.contains(player.as_str()) {
                // The player is already in the ranking, update their score.
                if let Some(player_entry) = ranking.iter_mut().find(|entry| entry.name == *player) {
                    player_entry.kills += kills;
                }
            } else {
                // The player is not in the ranking, add them.
                ranking.push(PlayerScore {
                    name: player.clone(),
                    kills: *kills,
                });
                player_set.insert(player.as_str());
            }
        }
    }

    // Sorts the Vec in descending order of kills.
    ranking.sort_by(|a, b| b.kills.cmp(&a.kills));
}
