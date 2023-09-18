use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Player {
    pub id: String,
    pub score: u32,
}
