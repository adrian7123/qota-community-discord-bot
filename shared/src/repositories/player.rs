use futures::TryStreamExt;
use mongodb::{bson::Document, options::AggregateOptions, Database};

use crate::models::player::Player;

static DB_NAME: &'static str = "players";

pub async fn get_players(
    db: Database,
    doc: impl IntoIterator<Item = Document>,
    options: impl Into<Option<AggregateOptions>>,
) -> Result<Vec<Player>, mongodb::error::Error> {
    let mut players: Vec<Player> = vec![];

    let mut cursor = db
        .collection::<Player>(DB_NAME)
        .aggregate(doc, options)
        .await?;

    while let Some(player) = cursor.try_next().await? {
        players.push(mongodb::bson::from_document(player)?)
    }

    Ok(players)
}
