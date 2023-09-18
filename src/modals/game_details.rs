use serde::Deserialize;

use super::{game_log::GameLog, player::Player};

#[derive(Deserialize, Debug)]
pub struct GameDetails {
    pub id: String,
    pub log: Vec<GameLog>,
    pub players: (Player, Player),
    #[serde(rename = "self")]
    pub self_id: String,
}
