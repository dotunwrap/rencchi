use crate::utils::db;
use crate::Context;
use futures::{Stream, StreamExt};
use mysql::prelude::*;
use mysql::*;

pub mod session;

pub struct Campaign {
    id: i64,
    dm_id: String,
    name: String,
    description: String,
}

impl From<i64> for Campaign {
    fn from(id: i64) -> Self {
        db::init_dnd_db()
            .exec_map(
                "SELECT id, dm_id, name, description
                FROM campaigns
                WHERE id = :id",
                params! { id },
                |(id, dm_id, name, description)| Self {
                    id,
                    dm_id,
                    name,
                    description,
                },
            )
            .expect("Failed to get campaign information")
            .pop()
            .expect("Campaign not found")
    }
}

pub async fn autocomplete_campaign<'a>(
    _ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    let campaigns: Vec<String> = db::init_dnd_db_async()
        .await
        .expect("Failed to connect to database")
        .query("SELECT DISTINCT name FROM campaigns")
        .unwrap();
    futures::stream::iter(campaigns)
        .filter(move |c| futures::future::ready(c.starts_with(partial)))
        .map(|c| c.to_string())
}

pub async fn get_id_from_name(name: String) -> i64 {
    db::init_dnd_db_async()
        .await
        .expect("Failed to connect to database")
        .exec_first(
            "SELECT id FROM campaigns WHERE name LIKE :name",
            params! { name },
        )
        .expect("Failed to get ID from name")
        .expect("Campaign not found")
}

pub async fn get_name_from_id(id: i64) -> String {
    db::init_dnd_db_async()
        .await
        .expect("Failed to connect to database")
        .exec_first("SELECT name FROM campaigns WHERE id = :id", params! { id })
        .expect("Failed to get name from ID")
        .expect("Campaign not found")
}

pub async fn does_campaign_exist(id: i64) -> bool {
    db::init_dnd_db_async()
        .await
        .expect("Failed to connect to database")
        .exec_first::<i64, _, _>("SELECT id FROM campaigns WHERE id = :id", params! { id })
        .expect("Failed to check if campaign exists")
        .is_some()
}
