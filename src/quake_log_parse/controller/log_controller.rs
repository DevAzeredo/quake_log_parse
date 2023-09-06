use crate::quake_log_parse::{model::{log_model::LogModel, error::LogError}, view::log_view::LogView};

pub struct LogController;

impl LogController {
    /// Reports match results and player rankings.
    ///
    /// This function calls the `LogModel::get_matches_and_player_rank` function to obtain information
    /// about the matches and player rankings. It then calls `LogView::render_matches_and_player_rank`
    /// to display the results.
    ///
    /// # Returns
    ///
    /// Returns a `Result` with the following meaning:
    ///
    /// - `Ok(())` - Indicates that the operation was successful, and the information was reported successfully.
    /// - `Err(LogError)` - Indicates that an error occurred while retrieving or displaying the information.
    ///   The `LogError` contains details about the encountered problem.
    pub fn report_match_and_player_rank() -> Result<(), LogError> {
        let (matches, player_rank) = LogModel::get_matches_and_player_rank()?;
        LogView::render_matches_and_player_rank(matches, player_rank)?;

        Ok(())
    }
}
