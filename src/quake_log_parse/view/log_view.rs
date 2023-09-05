use std::collections::HashMap;

use serde_json::{json, Value};

use crate::quake_log_parse::model::log_model::Match;
pub struct LogView {}
impl LogView {
    pub fn render_matches_and_player_rank(games: Vec<Match>, player_ranking: Vec<(String, i32)>) {
        let mut games_json = serde_json::Map::new();

        for game in games {
            let game_data = json!({
                "total_kills": game.data.total_kills,
                "players": game.data.players.iter().cloned().collect::<Vec<_>>(),
                "kills": game.data.kills.clone(),
            });

            games_json.insert(format!("game_{}", game.id.to_string()), game_data);
        }

        let json_player_ranking = json!({ "Player Ranking": player_ranking });
        println!("Renderizando partidas e Ranking");
        println!("{}", serde_json::to_string_pretty(&games_json).unwrap());
        println!("{}", json_player_ranking);
    }
}