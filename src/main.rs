#![warn(clippy::str_to_string)]

use crate::commands::*;
use dotenv::dotenv;
use poise::serenity_prelude as serenity;

pub mod commands;
pub mod responses;
pub mod utils;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
    pub welcome_role: u64,
    pub staff_role: u64,
    pub dnd_role: u64,
    pub general_channel: u64,
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) -> () {
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => {
            println!(
                "Error running command '{}': {:?}",
                ctx.command().name,
                error
            );

            responses::failure(ctx, "Something went wrong.")
                .await
                .unwrap_or_default();
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Error running built-in error handler: {:?}", e);
            }
        }
    }
}

async fn on_event(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot } => {
            println!("Logged in as {}", data_about_bot.user.name)
        }
        serenity::FullEvent::GuildMemberAddition { new_member } => {
            println!("New member: {}", new_member.user.name);

            if new_member.user.bot {
                return Ok(());
            }

            serenity::ChannelId::from(data.general_channel)
                .say(
                    ctx,
                    format!(
                        "<@&{}> New member: <@{}>",
                        serenity::RoleId::from(data.staff_role),
                        new_member.user.id,
                    ),
                )
                .await
                .unwrap();
        }
        _ => {}
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            event_handler: |_ctx, event, _framework, _data| {
                Box::pin(on_event(_ctx, event, _framework, _data))
            },
            commands: vec![
                help::help(),
                purge::purge(),
                welcome::welcome(),
                user::user_info(),
                dnd::campaign::session::session(),
                dnd::dice::roll(),
            ],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(".".into()),
                edit_tracker: Some(Into::into(poise::EditTracker::for_timespan(
                    std::time::Duration::from_secs(60),
                ))),
                ..Default::default()
            },
            on_error: |error| Box::pin(on_error(error)),
            command_check: Some(|ctx| {
                Box::pin(async move {
                    Ok(ctx
                        .author_member()
                        .await
                        .unwrap()
                        .roles
                        .contains(&serenity::RoleId::from(ctx.data().welcome_role)))
                })
            }),
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    welcome_role: 878085322469150720,
                    staff_role: 877709018204889128,
                    dnd_role: 901464574530814002,
                    general_channel: 877354423909756941,
                })
            })
        })
        .build();

    serenity::ClientBuilder::new(
        std::env::var("DISCORD_TOKEN").expect("Missing Discord token"),
        serenity::GatewayIntents::non_privileged()
            | serenity::GatewayIntents::MESSAGE_CONTENT
            | serenity::GatewayIntents::GUILD_MEMBERS,
    )
    .framework(framework)
    .await
    .unwrap()
    .start()
    .await
    .unwrap();
}

// Made with ❤ by dotunwrap
