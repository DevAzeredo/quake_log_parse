use crate::quake_log_parse::{
    model::{error::LogError, log_model::LogModel},
    view::log_view::LogView,
};

pub struct LogController;

impl LogController {
    /// Generates and renders a game report.
    ///
    /// This function generates a game report by first processing the game log using `LogModel::process_log()`
    /// to obtain information about game matches, player rankings, and means of kills. It then renders the report using
    /// `LogView::render_report()`.
    ///
    /// # Returns
    ///
    /// * `Result<(), LogError>` - A `Result` indicating success (`Ok`) or an error (`Err`) if any problem
    ///   occurs during report generation or rendering.
    ///
    /// # Errors
    ///
    /// Returns an error of type `LogError` if there are any issues with processing the log or rendering the report.
    pub fn generate_and_render_report() -> Result<(), LogError> {
        let (matches, player_rank) = LogModel::process_log()?;
        LogView::render_report(matches, player_rank)?;

        Ok(())
    }
}
