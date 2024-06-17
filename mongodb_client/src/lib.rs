use std::env;

use dotenv::dotenv;
use mongodb::{Client, Database};
use serenity::prelude::TypeMapKey;

pub struct MongoDBClient {
    pub client: Client,
    pub database: Database,
}

impl TypeMapKey for MongoDBClient {
    type Value = Self;
}

impl MongoDBClient {
    pub async fn new() -> Result<Self, mongodb::error::Error> {
        dotenv().ok();

        let mongodb_uri = env::var("MONGODB_URI").expect("MONGODB_URI err");
        let mongodb_db_name = env::var("MONGODB_DB_NAME").expect("MONGODB_DB_NAME err");

        let client = Client::with_uri_str(mongodb_uri).await?;

        let database = client.database(mongodb_db_name.as_str());

        Ok(Self { client, database })
    }
}
