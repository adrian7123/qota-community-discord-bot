use ::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    #[serde(rename = "_id")]
    pub id: String,
    pub discord_id: String,
}
