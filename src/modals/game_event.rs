use serde::Deserialize;

use super::game_details::GameDetails;

#[derive(Deserialize, Debug)]
pub struct GameInitEvent {
    #[serde(flatten)]
    pub details: GameDetails,
}

#[derive(Deserialize, Debug)]
pub struct GameResultEvent {
    #[serde(flatten)]
    pub details: GameDetails,
}

#[derive(Deserialize, Debug)]
pub struct GameRoundEvent {
    pub round: u32,
    #[serde(flatten)]
    pub details: GameDetails,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum GameEvent {
    #[serde(rename = "INIT")]
    Init(GameInitEvent),
    #[serde(rename = "RESULT")]
    Result(GameResultEvent),
    #[serde(rename = "ROUND")]
    Round(GameRoundEvent),
}
