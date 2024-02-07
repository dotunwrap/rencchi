use crate::{responses, Context, Error};
use poise::serenity_prelude as serenity;

/// Gets info regarding a user
#[poise::command(
    context_menu_command = "User info",
    prefix_command,
    slash_command,
    category = "User"
)]
pub async fn user_info(
    ctx: Context<'_>,
    #[description = "User to get info on"] user: serenity::User,
) -> Result<(), Error> {
    let avatar = user.face();
    let created_at = user.created_at().format("%Y-%m-%d %H:%M:%S");

    ctx.send(
        poise::reply::CreateReply::default().embed(
            serenity::CreateEmbed::new()
                .title(format!("{}'s info", user.name))
                .image(avatar)
                .field("Name", user.name, false)
                .field("ID", user.id.to_string(), false)
                .field("Created at", created_at.to_string(), false)
                .footer(serenity::CreateEmbedFooter::new(format!(
                    "Requested by {}",
                    ctx.author().name
                ))),
        ),
    )
    .await?;

    Ok(())
}

#[poise::command(
    context_menu_command = "Who is",
    prefix_command,
    slash_command,
    category = "User"
)]
pub async fn whois(ctx: Context<'_>, user: serenity::User) -> Result<(), Error> {
    let mut irl_name: Option<String> = None;
    let nickname = match ctx.guild_id() {
        Some(guild_id) => user.nick_in(ctx, guild_id).await,
        None => None,
    };
    let discord_name = nickname.as_ref().unwrap_or(&user.name);

    if irl_name.is_none() {
        return responses::failure(
            ctx,
            &format!("{} has not set their IRL name.", discord_name),
        )
        .await;
    }

    let irl_name = "";

    responses::success(ctx, &format!("{} is {}", discord_name, irl_name)).await
}

#[poise::command(prefix_command, slash_command, category = "User")]
pub async fn iam(ctx: Context<'_>, name: String) -> Result<(), Error> {
    responses::success(ctx, &format!("Name updated! You are {}", name)).await
}
