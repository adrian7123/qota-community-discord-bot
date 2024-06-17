use chrono::{DateTime, Timelike, Utc};
use futures::{StreamExt, TryStreamExt};
use mongodb::{
    bson::{doc, oid::ObjectId, Uuid},
    options::FindOneOptions,
    Collection, Database,
};
use serenity::utils::MessageBuilder;

use crate::models::{mix::Mix, mix_schedule::MixSchedule, player::Player};

#[derive(Debug, Clone)]
pub struct MixHelper {
    mix: Collection<Mix>,
    schedule: Collection<MixSchedule>,
}

impl MixHelper {
    pub async fn new(db: Database) -> Self {
        Self {
            mix: db.collection("mix"),
            schedule: db.collection("mix_schedule"),
        }
    }
    pub fn make_message_mix_list(&self, mix: Mix, players: Vec<Player>) -> MessageBuilder {
        let mut message: MessageBuilder = MessageBuilder::new();

        message
            .push("Mix Que Ota Community ")
            .push(format!("{}", mix.date.format("**%d/%m** ")))
            .push(format!("{}", mix.date.format("**%H:%M** ")))
            .push("\n\n");
        let mut pos: u8 = 0;
        for player in players {
            pos += 1;
            message.push_bold(format!("{}  -  <@{}>", pos, player.discord_id));
            message.push("\n");
        }

        message.push("\n");

        message
    }
    pub fn get_current_date(&self, hour: Option<u32>, min: Option<u32>) -> DateTime<Utc> {
        let mut h: u32 = 0;
        let mut m: u32 = 0;

        if hour.is_some() {
            h = hour.unwrap();
        }
        if min.is_some() {
            m = min.unwrap();
        }

        chrono::Utc::now()
            .with_hour(h)
            .unwrap()
            .with_minute(m)
            .unwrap()
            .with_second(0)
            .unwrap()
            .with_nanosecond(0)
            .unwrap()
    }
    pub async fn mix_is_created(&self) -> (bool, MessageBuilder) {
        let mixes = self.get_mix_many().await.expect("msg");
        let mut message = MessageBuilder::new();
        let mut created = false;
        if mixes.is_empty() {
            message
                .push("Lista de espera ainda não foi criada 😐")
                .push("Digite !criarlista **22:00** para criar uma nova lista.📅");
        } else {
            created = true;
            message
                .push("Lista já foi criada 🗓️.\n")
                .push("Digite !cancelarlista 💀 para remover lista atual.")
                .build();
        }
        (created, message)
    }
    pub async fn create_mix(&self, current_date: DateTime<Utc>) -> Mix {
        let mix = Mix {
            id: ObjectId::new(),
            date: current_date,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            expired: false,
            cron: vec![],
            players: vec![],
        };

        self.mix.insert_one(&mix, None).await.expect("asd");

        mix
    }
    pub async fn create_mix_schedule(
        &self,
        uuid: Uuid,
        mix: ObjectId,
        schedule: String,
    ) -> MixSchedule {
        let schedule = MixSchedule {
            id: ObjectId::new(),
            uuid: uuid.to_string(),
            mix,
            schedule,
            executed: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        // self.schedule.insert_one(schedule, None).await.expect("asd");

        schedule
    }
    /// Update cronjob of mix
    pub async fn update_mix_schedule(&self, uuid: String, params: Vec<String>) {
        // self.db
        //     .mix_schedule()
        //     .update(mix_schedule::id::equals(uuid), params)
        //     .exec()
        //     .await
        //     .unwrap();
    }
    /// Get all cronjob of mix
    pub async fn get_mix_schedule_many_by_mix_id(&self, mix_id: String) {
        // self.db
        //     .mix_schedule()
        //     .find_many(vec![mix_schedule::mix_id::equals(mix_id)])
        //     .exec()
        //     .await
        //     .unwrap()
    }
    pub async fn get_mix_many(&self) -> mongodb::error::Result<Vec<Mix>> {
        let mut mixes: Vec<Mix> = vec![];

        let mut cursor = self.mix.find(None, None).await?;

        while let Some(result) = cursor.next().await {
            match result {
                Ok(doc) => {
                    mixes.push(doc);
                }
                Err(e) => {
                    // Handle error from cursor
                    eprintln!("Error fetching document: {:?}", e);
                }
            }
        }

        Ok(mixes)
    }
    pub async fn get_current_mix(&self) -> Option<Mix> {
        self.mix
            .find_one(
                None,
                FindOneOptions::builder()
                    .sort(doc! { "updated_at": -1 })
                    .build(),
            )
            .await
            .expect("erro")
    }
    pub async fn get_mix_players(&self, mix_id: String) {
        // self.db
        //     .mix_player()
        //     .find_many(vec![mix_player::mix_id::equals(Some(mix_id))])
        //     .order_by(mix_player::created_at::order(Direction::Asc))
        //     .exec()
        //     .await
        //     .unwrap()
    }
    pub async fn create_mix_player(
        &self,
        name: String,
        discord_id: String,
        create_params: Vec<&'static str>,
    ) {
        // self.db
        //     .mix_player()
        //     .create(name, discord_id, create_params)
        //     .exec()
        //     .await
        //     .expect("err ao criar player")
    }
    pub async fn get_mix_player(&self, _where: Vec<&'static str>) {
        // self.db
        //     .mix_player()
        //     .find_first(_where)
        //     .exec()
        //     .await
        //     .unwrap()
    }
    pub async fn cancel_current_mix(&self, mix_id: ObjectId) {
        // let _ = self
        //     .db
        //     .mix()
        //     .update(mix::id::equals(mix_id), vec![mix::expired::set(true)])
        //     .exec()
        //     .await;
    }
    pub async fn delete_all_mix_players(&self, mix_id: String) {
        // self.db
        //     .mix_player()
        //     .delete_many(vec![mix_player::mix_id::equals(Some(mix_id))])
        //     .exec()
        //     .await
        //     .unwrap();
    }

    pub async fn delete_mix_player(&self, discord_id: String, mix_id: String) {
        // self.db
        //     .mix_player()
        //     .delete_many(vec![
        //         mix_player::discord_id::equals(discord_id),
        //         mix_player::mix_id::equals(Some(mix_id)),
        //     ])
        //     .exec()
        //     .await
        //     .unwrap();
    }
}
