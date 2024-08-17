use poise::serenity_prelude as serenity;

use crate::responses::invalid_permissions;
use crate::{Context, Error};

pub async fn author_is_staff(ctx: Context<'_>) -> Result<bool, Error> {
    let member = ctx.author_member().await.ok_or_else(|| {
        Error::from("Failed to get the author's member. This should never happen.")
    })?;

    Ok(member.roles.contains(&serenity::RoleId::from(
        ctx.data().config.roles.staff_role_id,
    )))
}

pub async fn has_staff_role_check(ctx: Context<'_>) -> Result<bool, Error> {
    if !author_is_staff(ctx).await? {
        invalid_permissions(ctx).await?;
        return Ok(false);
    }

    Ok(true)
}
