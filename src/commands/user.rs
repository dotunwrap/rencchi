use crate::{Context, Error};
use poise::serenity_prelude as serenity;

#[poise::command(context_menu_command = "User info", prefix_command, slash_command)]
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
