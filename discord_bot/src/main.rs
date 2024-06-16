use discord_bot::serenity_start;
use dotenv::dotenv;
use std::env;

use mongodb::{
    bson::{doc, Document},
    Client, Collection,
};

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    dotenv().ok();

    let mongodb_uri = env::var("MONGODB_URI").expect("MONGODB_URI err");
    let mongodb_db_name = env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME err");

    let client = Client::with_uri_str(mongodb_uri).await?;

    let database = client.database(mongodb_db_name);

    serenity_start(env::var("DISCORD_TOKEN").expect("token")).await;
}
