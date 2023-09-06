use serde_json::json;

use crate::quake_log_parse::model::{
    error::LogError,
    log_model::{Match, PlayerScore},
};
pub struct LogView {}
impl LogView {
    /// Renders match data and player rankings to the console.
    ///
    /// This function takes a vector of matches and a vector of player rankings,
    /// displays the match data and the player rankings.
    ///
    /// # Arguments
    ///
    /// * `games` - A vector of `Match` objects representing match data.
    /// * `player_ranking` - A vector of `PlayerScore` objects representing player rankings.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with the following meaning:
    ///
    /// - `Ok(())` - Indicates that the rendering operation was successful.
    /// - `Err(LogError)` - Indicates that an error occurred during rendering
    pub fn render_matches_and_player_rank(
        games: Vec<Match>,
        player_ranking: Vec<PlayerScore>,
    ) -> Result<(), LogError> {
        println!("Renderizando partidas e Ranking");
        render_matches(games)?;
        render_ranking(player_ranking)?;
        Ok(())
    }
}
/// Renders the player ranking to the console.
///
/// This function takes a vector of `PlayerScore` objects and displays the player ranking.
///
/// # Arguments
///
/// * `player_ranking` - A vector of `PlayerScore` objects representing player rankings.
///
/// # Returns
///
/// Returns a `Result` with the following meaning:
///
/// - `Ok(())` - Indicates that the rendering operation was successful.
/// - `Err(LogError)` - Indicates that an error occurred during rendering.
fn render_ranking(player_ranking: Vec<PlayerScore>) -> Result<(), LogError> {
    let ranking: Vec<_> = player_ranking
        .iter()
        .map(|player| json!({&player.name:player.kills}))
        .collect();

    println!(
        "{}",
        serde_json::to_string_pretty(&json!({ "Player Ranking": ranking })).unwrap()
    );
    Ok(())
}
/// Renders the match data to the console.
///
/// This function takes a vector of `Match` objects and displays the match data.
///
/// # Arguments
///
/// * `games` - A vector of `Match` objects representing match data.
///
/// # Returns
///
/// Returns a `Result` with the following meaning:
///
/// - `Ok(())` - Indicates that the rendering operation was successful.
/// - `Err(LogError)` - Indicates that an error occurred during rendering
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
                    })
            })
        })
        .collect();

    println!(
        "{}",
        serde_json::to_string_pretty(&json!({ "matches": mat })).unwrap()
    );
    Ok(())
}
