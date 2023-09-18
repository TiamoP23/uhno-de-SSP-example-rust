use rand::distributions::{Distribution, Standard};
use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub enum GameMove {
    #[serde(rename = "PAPIER")]
    Paper,
    #[serde(rename = "STEIN")]
    Rock,
    #[serde(rename = "SCHERE")]
    Scissors,
}

impl Distribution<GameMove> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> GameMove {
        match rng.gen_range(0..3) {
            0 => GameMove::Paper,
            1 => GameMove::Rock,
            2 => GameMove::Scissors,
            _ => unreachable!(),
        }
    }
}
