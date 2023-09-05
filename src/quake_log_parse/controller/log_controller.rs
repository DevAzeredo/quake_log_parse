use crate::quake_log_parse::{model::log_model::LogModel, view::log_view::LogView};

pub struct LogController {}

impl LogController {
    pub fn report_match_and_player_rank() {
        // Implementar a lógica para relatar a partida e o ranking dos jogadores
        let (matchs, player_rank) = LogModel::get_matches_and_player_rank();
        LogView::render_matches_and_player_rank(matchs, player_rank);
    }
}
