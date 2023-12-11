use crate::{responses, utils::staff, Context, Error};
use poise::serenity_prelude as serenity;

#[poise::command(
    slash_command,
    prefix_command,
    required_bot_permissions = "MANAGE_ROLES"
)]
pub async fn welcome(ctx: Context<'_>, member: serenity::Member) -> Result<(), Error> {
    if !staff::author_is_staff(ctx).await.unwrap() {
        return responses::not_staff(ctx).await;
    }

    if member.user.bot {
        return responses::failure(ctx, &format!("Error: <@{}> is a bot.", member.user.id)).await;
    }

    if member
        .roles
        .contains(&serenity::RoleId::from(ctx.data().welcome_role))
    {
        return responses::failure(
            ctx,
            &format!("Error: <@{}> already has guest role.", member.user.id),
        )
        .await;
    }

    member
        .add_role(ctx, serenity::RoleId::from(ctx.data().welcome_role))
        .await
        .unwrap();
    responses::success(ctx, &format!("Welcome, <@{}>!", member.user.id)).await
}
