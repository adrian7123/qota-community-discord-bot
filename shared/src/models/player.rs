use ::serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub discord_id: String,
    pub mix_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
