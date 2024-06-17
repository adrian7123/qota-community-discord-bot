use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MixSchedule {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub uuid: String,
    pub schedule: String,
    pub executed: bool,
    pub mix: ObjectId,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
