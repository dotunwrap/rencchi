use crate::{responses, utils::db, Context, Error};
use chrono::{Local, NaiveDateTime, TimeZone};
use poise::serenity_prelude as serenity;

pub mod response;

static DND_DB: &str = "dnd.db";

#[derive(Debug)]
struct Session {
    author_id: String,
    status: i64,
    created_date: String,
    scheduled_date: String,
}

impl Session {
    fn new(author_id: String, status: i64, created_date: String, scheduled_date: String) -> Self {
        Self {
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

/// D&D Sessions (subcommand required)
///
/// (SLASH | PREFIX) session <subcommand>
#[poise::command(
    prefix_command,
    slash_command,
    subcommands("create", "cancel", "list"),
    subcommand_required,
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
    #[description = "Scheduled date"] scheduled_date: String,
) -> Result<(), Error> {
    let mut session = Session::new(
        ctx.author().id.to_string(),
        0,
        Local::now().to_string(),
        scheduled_date,
    );

    let scheduled_date = NaiveDateTime::parse_from_str(&session.scheduled_date, "%Y-%m-%d %H:%M");

    if scheduled_date.is_err() {
        responses::failure(ctx, "Invalid date format.").await?
    }

    let scheduled_date = Local.from_local_datetime(&scheduled_date.unwrap()).unwrap();

    if scheduled_date < Local::now() {
        responses::failure(ctx, "Scheduled date must be in the future.").await?
    }

    session.scheduled_date = scheduled_date.to_string();

    let conn = db::init_db(DND_DB).await?;

    for row in conn
        .prepare(
            "INSERT INTO sessions (
                author_id,
                status,
                created_date,
                scheduled_date
            ) VALUES (
                :author_id,
                :status,
                :created_date,
                :scheduled_date
            )",
        )?
        .into_iter()
        .bind((":author_id", &session.author_id[..]))?
        .bind((":status", session.status))?
        .bind((":created_date", &session.created_date[..]))?
        .bind((":scheduled_date", &session.scheduled_date[..]))?
        .map(|row| row.unwrap())
    {
        println!("{:?}", row);
    }

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
    let conn = db::init_db(DND_DB).await?;

    for row in conn
        .prepare("DELETE FROM sessions")?
        .into_iter()
        .map(|row| row.unwrap())
    {
        println!("{:?}", row);
    }

    responses::success(ctx, "All sessions deleted.").await
}

/// Lists all D&D sessions
///
/// (SLASH | PREFIX) session list
#[poise::command(prefix_command, slash_command)]
pub async fn list(_ctx: Context<'_>) -> Result<(), Error> {
    let conn = db::init_db(DND_DB).await?;
    let mut embeds: Vec<serenity::CreateEmbed> = vec![];
    let query = "SELECT author_id, created_date, scheduled_date, status, author_id FROM sessions WHERE scheduled_date > :now ORDER BY scheduled_date ASC";

    for row in conn
        .prepare(query)?
        .into_iter()
        .bind((":now", &Local::now().to_string()[..]))?
        .map(|row| row.unwrap())
    {
        let session = Session::new(
            row.read::<i64, _>("author_id").to_string(),
            row.read::<i64, _>("status"),
            row.read::<&str, _>("created_date").to_string(),
            row.read::<&str, _>("scheduled_date").to_string(),
        );

        embeds.push(
            serenity::CreateEmbed::new()
                .title(format!(
                    "{}",
                    NaiveDateTime::parse_from_str(
                        &session.scheduled_date[..],
                        "%Y-%m-%d %H:%M:%S %:z"
                    )?
                    .format("%Y-%m-%d %H:%M")
                ))
                .field("Status", session.status_translation(), false),
        );
    }

    // responses::paginate(ctx, embeds).await;

    Ok(())
}
