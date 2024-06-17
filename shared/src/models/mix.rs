use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use my_macro::DbActions;
use serde::{Deserialize, Serialize};

use super::{mix_schedule::MixSchedule, player::Player};

#[derive(Serialize, Deserialize, Debug, DbActions)]
pub struct Mix {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub date: DateTime<Utc>,
    pub expired: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub cron: Vec<MixSchedule>,
    pub players: Vec<Player>,
}

trait DB {}
