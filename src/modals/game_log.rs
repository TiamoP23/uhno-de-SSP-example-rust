use serde::Deserialize;

use super::game_move::GameMove;

#[derive(Deserialize, Debug)]
pub struct GameLog {
    pub rating: (u32, u32),
    pub results: (Option<GameMove>, Option<GameMove>),
    pub round: u32,
}
