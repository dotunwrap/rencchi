use anyhow::Context as _;
use poise::serenity_prelude as serenity;
use serde::Deserialize;
use shuttle_runtime::SecretStore;
use shuttle_serenity::ShuttleSerenity;
use std::{env::current_dir, fs::read_to_string, process::exit};

use crate::commands::*;
use crate::utils::welcome::send_welcome_message;

pub mod commands;
pub mod responses;
pub mod utils;

#[derive(Deserialize)]
pub struct GeneralConfig {
    pub prefix: String,
}

#[derive(Deserialize)]
pub struct ChannelConfig {
    pub new_user_channel_id: u64,
    pub general_channel_id: u64,
}

#[derive(Deserialize)]
pub struct RoleConfig {
    pub new_user_role_id: u64,
    pub welcomed_user_role_id: u64,
    pub welcome_party_role_id: u64,
    pub staff_role_id: u64,
}

#[derive(Deserialize)]
pub struct Config {
    pub general: GeneralConfig,
    pub channels: ChannelConfig,
    pub roles: RoleConfig,
}

pub struct Data {
    config: Config,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
type _ApplicationContext<'a> = poise::ApplicationContext<'a, Data, Error>;

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => {
            panic!("Failed to build framework: {:?}", error)
        }
        poise::FrameworkError::Command { error, ctx, .. } => {
            println!(
                "Command '{}' returned an error: {}",
                ctx.command().qualified_name,
                error
            );

            responses::failure(ctx, "Something went wrong.")
                .await
                .unwrap_or_default();
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("Failed to call on_error: {}", e);
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
            println!("{} is connected!", data_about_bot.user.name)
        }
        serenity::FullEvent::GuildMemberAddition { new_member } => {
            send_welcome_message(ctx, data, new_member.user.clone()).await?
        }
        _ => (),
    }
    Ok(())
}

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secret_store: SecretStore) -> ShuttleSerenity {
    // Handle loading the config
    let config_file = current_dir()?.join("src/config.toml");

    let contents = match read_to_string(config_file.clone()) {
        Ok(c) => c,
        Err(e) => {
            eprintln!(
                "Could not read config file {}: {}",
                config_file.display(),
                e
            );
            exit(1);
        }
    };

    let config: Config = match toml::from_str(&contents) {
        Ok(c) => c,
        Err(e) => {
            eprintln!(
                "Could not parse config file {}: {}",
                config_file.display(),
                e
            );
            exit(1);
        }
    };

    // The Discord token is stored in Secrets.toml
    // Shuttle can also read a dev token from Secrets.dev.toml
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let commands = vec![misc::help(), users::user_info()];

    let framework = poise::Framework::<Data, Error>::builder()
        .options(poise::FrameworkOptions {
            event_handler: |_ctx, event, _framework, _data| {
                Box::pin(on_event(_ctx, event, _framework, _data))
            },
            commands,
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(config.general.prefix.clone()),
                ..Default::default()
            },
            on_error: |error| Box::pin(on_error(error)),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { config })
            })
        })
        .build();

    let intents = serenity::GatewayIntents::non_privileged()
        | serenity::GatewayIntents::MESSAGE_CONTENT
        | serenity::GatewayIntents::GUILD_MEMBERS;

    let client = serenity::ClientBuilder::new(discord_token, intents)
        .framework(framework)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(client.into())
}
