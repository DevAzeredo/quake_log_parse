use serde_json::json;

use crate::quake_log_parse::model::{
    error::LogError,
    log_model::{Match, PlayerScore},
};
pub struct LogView {}
impl LogView {
    /// Renders game matches and player rankings to the output.
    ///
    /// This function takes a vector of `Match` structs representing game matches and a vector of `PlayerScore`
    /// structs representing player rankings. It then prints these data to the console
    ///
    /// # Arguments
    ///
    /// * `games` - A vector of `Match` structs containing information about game matches to be rendered.
    ///
    /// * `player_ranking` - A vector of `PlayerScore` structs containing player rankings to be rendered.
    ///
    /// # Returns
    ///
    /// * `Result<(), LogError>` - A `Result` indicating success (`Ok`) or an error (`Err`) if any problem
    ///   occurs during rendering.
    ///
    /// # Errors
    ///
    /// Returns an error of type `LogError` if there are any issues with rendering the game matches or player rankings.
    pub fn render_report(
        games: Vec<Match>,
        player_ranking: Vec<PlayerScore>,
    ) -> Result<(), LogError> {
        render_matches(games)?;
        render_ranking(player_ranking)?;
        Ok(())
    }
}
/// Renders player rankings to the output in JSON format.
///
/// This function takes a vector of `PlayerScore` structs representing player rankings and renders
/// them to the output in a JSON format. It prints the JSON representation to the console
///
/// # Arguments
///
/// * `player_ranking` - A vector of `PlayerScore` structs containing player rankings to be rendered.
///
/// # Returns
///
/// * `Result<(), LogError>` - A `Result` indicating success (`Ok`) or an error (`Err`) if any problem
///   occurs during rendering.
///
/// # Errors
///
/// Returns an error of type `LogError` if there are any issues with rendering the player rankings.
fn render_ranking(player_ranking: Vec<PlayerScore>) -> Result<(), LogError> {
    let ranking: Vec<_> = player_ranking
        .iter()
        .map(|player| json!({&player.name:player.kills}))
        .collect();

    println!(
        "{}",
        serde_json::to_string_pretty(&json!({ "Player Ranking": ranking }))?
    );
    Ok(())
}

/// Renders game match data to the output in JSON format.
///
/// This function takes a vector of `Match` structs representing game matches and renders
/// them prints the JSON representation to the console.
///
/// # Arguments
///
/// * `games` - A vector of `Match` structs containing game match data to be rendered.
///
/// # Returns
///
/// * `Result<(), LogError>` - A `Result` indicating success (`Ok`) or an error (`Err`) if any problem
///   occurs during rendering.
///
/// # Errors
///
/// Returns an error of type `LogError` if there are any issues with rendering the game match data.
fn render_matches(games: Vec<Match>) -> Result<(), LogError> {
    let mat: Vec<_> = games
        .iter()
        .map(|game| {
            json!({
                "game_".to_owned()
                    + &game.id.to_string(): json!({
                      "total_kills": game.data.total_kills,
                        "players": game.data.players,
                        "kills": game.data.kills,
                        "death_causes": game.data.kills_by_means,
                    })
            })
        })
        .collect();

    println!(
        "{}",
        serde_json::to_string_pretty(&json!({ "matches": mat }))?
    );
    Ok(())
}
