use crate::{Context, Error};
use poise::serenity_prelude as serenity;

pub async fn author_is_staff(ctx: Context<'_>) -> Result<bool, Error> {
    Ok(ctx
        .author_member()
        .await
        .unwrap()
        .roles
        .contains(&serenity::RoleId::from(ctx.data().staff_role)))
}
