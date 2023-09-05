use serde_json::json;

use crate::quake_log_parse::model::log_model::{Match, PlayerScore};
pub struct LogView {}
impl LogView {
    pub fn render_matches_and_player_rank(games: Vec<Match>, player_ranking: Vec<PlayerScore>) {
        println!("Renderizando partidas e Ranking");
        render_matches(games);
        render_ranking(player_ranking);
    }
}
fn render_ranking(player_ranking: Vec<PlayerScore>) {
    let ranking: Vec<_> = player_ranking
        .iter()
        .map(|player| json!({&player.name:player.kills}))
        .collect();

    println!(
        "{}",
        serde_json::to_string_pretty(&json!({ "Player Ranking": ranking })).unwrap()
    );
}

fn render_matches(games: Vec<Match>) {
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
}
