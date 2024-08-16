use poise::serenity_prelude as serenity;

use crate::{Data, Error};

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
