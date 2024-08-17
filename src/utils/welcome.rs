use poise::serenity_prelude as serenity;

use crate::responses::invalid_permissions;
use crate::{Context, Data, Error};

pub async fn has_welcoming_party_role_check(ctx: Context<'_>) -> Result<bool, Error> {
    let welcoming_party_role =
        serenity::RoleId::from(ctx.data().config.roles.welcome_party_role_id);

    let member = ctx.author_member().await.ok_or_else(|| {
        Error::from("Failed to get the author's member. This should never happen.")
    })?;

    if !member.roles.contains(&welcoming_party_role) {
        invalid_permissions(ctx).await?;
        return Ok(false);
    }

    Ok(true)
}

pub async fn send_welcome_message(
    ctx: &serenity::Context,
    data: &Data,
    user: serenity::User, // User who joined
) -> Result<(), Error> {
    let channel = serenity::ChannelId::from(data.config.channels.new_user_channel_id);
    let welcoming_party_role = serenity::RoleId::from(data.config.roles.welcome_party_role_id);
    channel
        .say(
            ctx,
            format!(
                "Welcome <@{}>! Someone from our <@&{}> will be with you shortly.",
                user.id, welcoming_party_role
            ),
        )
        .await?;

    Ok(())
}
