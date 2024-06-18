#[macro_use]
extern crate rocket;

mod controllers;
mod models;

use std::{env, thread, time::Duration};

use controllers::players;
use dotenv::dotenv;
use rocket::{http::Status, Request};

use serde_json::{json, Value};
use serenity::Client;
mod cors;

pub type Ctx = rocket::State<RocketContext>;

pub struct RocketContext {
    pub discord_client: Client,
}

#[catch(default)]
fn default(status: Status, req: &Request) -> Value {
    json!({ "status": status.code, "url": req.uri(), })
}

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    let token: String = env::var("DISCORD_TOKEN").expect("token");

    // ? start discord bot
    tokio::spawn(discord_bot::serenity_start(token.clone()));

    rocket::build()
        .attach(cors::CORS)
        .manage(RocketContext {
            discord_client: discord_bot::serenity_instance(token).await,
        })
        .register("/", catchers![default])
        .mount("/players", players::routes())
}
