use crate::{commands::dnd, responses, utils::db, Context, Error};
use chrono::{Local, NaiveDateTime, TimeZone};
use mysql::prelude::*;
use mysql::*;
use poise::serenity_prelude as serenity;

pub mod response;

struct Session {
    campaign: dnd::campaign::Campaign,
    author_id: String,
    status: i64,
    created_date: String,
    scheduled_date: String,
}

impl Session {
    fn new(
        campaign_id: i64,
        author_id: String,
        status: i64,
        created_date: String,
        scheduled_date: String,
    ) -> Self {
        let campaign = dnd::campaign::Campaign::from(campaign_id);
        Self {
            campaign,
            author_id,
            status,
            created_date,
            scheduled_date,
        }
    }

    fn status_translation(&self) -> String {
        match self.status {
            0 => String::from("Pending"),
            1 => String::from("Accepted"),
            2 => String::from("Cancelled"),
            _ => String::from("Unknown"),
        }
    }
}

impl From<i64> for Session {
    fn from(id: i64) -> Self {
        db::init_dnd_db()
            .exec_map(
                "SELECT campaign_id, author_id, status, created_date, scheduled_date
                FROM sessions
                WHERE id = :id",
                params! { id },
                |(campaign_id, author_id, status, created_date, scheduled_date)| {
                    Self::new(campaign_id, author_id, status, created_date, scheduled_date)
                },
            )
            .expect("Failed to get session information")
            .pop()
            .expect("Session not found")
    }
}

/// D&D Sessions (subcommand required)
///
/// (SLASH | PREFIX) session <subcommand>
#[poise::command(
    prefix_command,
    slash_command,
    subcommands("create", "cancel", "clear_all", "list"),
    subcommand_required,
    check = "dnd::dnd_check",
    category = "D&D"
)]
pub async fn session(_ctx: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Creates a new D&D session
///
/// (SLASH | PREFIX) session create <scheduled_date>
/// The scheduled date must be in the future
#[poise::command(prefix_command, slash_command)]
pub async fn create(
    ctx: Context<'_>,
    #[description = "Campaign"]
    #[autocomplete = dnd::campaign::autocomplete_campaign]
    campaign: String,
    #[description = "Scheduled date"] scheduled_date: String,
) -> Result<(), Error> {
    ctx.defer().await?;
    let created_date = Local::now();
    let scheduled_date = NaiveDateTime::parse_from_str(&scheduled_date, "%Y-%m-%d %H:%M");

    if scheduled_date.is_err() {
        responses::failure(ctx, "Invalid date format.").await?
    }

    let scheduled_date = Local.from_local_datetime(&scheduled_date.unwrap()).unwrap();

    if scheduled_date < created_date {
        responses::failure(ctx, "Scheduled date must be in the future.").await?
    }

    let session = Session::new(
        dnd::campaign::get_id_from_name(campaign).await,
        ctx.author().id.to_string(),
        0,
        created_date.format("%Y-%m-%d %H:%M").to_string(),
        scheduled_date.format("%Y-%m-%d %H:%M").to_string(),
    );

    db::init_dnd_db_async()
        .await
        .expect("Failed to connect to database")
        .exec_drop(
            r"INSERT INTO sessions (
            campaign_id,
            author_id,
            status,
            created_date,
            scheduled_date
            ) VALUES (
                :campaign_id,
                :author_id,
                :status,
                :created_date,
                :scheduled_date
            )",
            params! {
                "campaign_id" => &session.campaign.id,
                "author_id" => &session.author_id,
                "status" => session.status,
                "created_date" => &session.created_date,
                "scheduled_date" => &session.scheduled_date
            },
        )?;

    responses::success(ctx, "Session created.").await
}

/// Cancels a D&D session
///
/// (SLASH | PREFIX) session cancel <session_id>
#[poise::command(prefix_command, slash_command)]
pub async fn cancel(
    _ctx: Context<'_>,
    #[description = "Session ID"] _session_id: i64,
) -> Result<(), Error> {
    todo!()
}

/// Deletes all D&D sessions (owner only)
///
/// (SLASH | PREFIX) session clear_all
#[poise::command(prefix_command, slash_command, owners_only)]
pub async fn clear_all(ctx: Context<'_>) -> Result<(), Error> {
    db::init_dnd_db_async()
        .await
        .expect("Failed to connect to database")
        .query_drop("TRUNCATE sessions")?;
    responses::success(ctx, "All sessions deleted.").await
}

/// Lists all D&D sessions
///
/// (SLASH | PREFIX) session list
#[poise::command(prefix_command, slash_command)]
pub async fn list(
    ctx: Context<'_>,
    #[autocomplete = dnd::campaign::autocomplete_campaign]
    #[description = "Campaign"]
    campaign: Option<String>,
) -> Result<(), Error> {
    ctx.defer().await?;
    let mut embeds: Vec<serenity::CreateEmbed> = vec![];
    let mut where_clause = String::from("WHERE scheduled_date > NOW()");
    let mut params = Params::Empty;

    match campaign {
        Some(campaign) => {
            where_clause = format!(
                "{} AND campaign_id = (SELECT campaign_id FROM campaigns WHERE name = :campaign)",
                where_clause
            );
            params = params! {
                campaign
            };
        }
        None => {}
    }

    db::init_dnd_db_async()
        .await
        .expect("Failed to connect to database")
        .exec_map(
            format!(
                "SELECT campaign_id, author_id, status, DATE_FORMAT(created_date, '%Y-%m-%d %H:%i') AS created_date, DATE_FORMAT(scheduled_date, '%Y-%m-%d %H:%i') AS scheduled_date
                FROM sessions
                {}
                {}",
                where_clause,
                "ORDER BY scheduled_date ASC"
            ),
            params,
            |(campaign_id, author_id, status, created_date, scheduled_date): (
                i64,
                String,
                i64,
                String,
                String
            )| {
                let session =
                    Session::new(campaign_id, author_id, status, created_date, scheduled_date);
                embeds.push(
                    serenity::CreateEmbed::new()
                        .title(format!(
                            "{}",
                            session.scheduled_date
                        ))
                        .field("Status", session.status_translation(), false),
                );
            },
        )
        .expect("Failed to get sessions");

    responses::paginate_embeds(ctx, embeds).await
}
