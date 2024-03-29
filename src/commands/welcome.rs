use crate::{responses, utils::staff, Context, Error};
use poise::serenity_prelude as serenity;

/// Welcomes a new user and gives them the guest role (staff only)
#[poise::command(
    slash_command,
    prefix_command,
    check = "staff::staff_check",
    required_bot_permissions = "MANAGE_ROLES",
    category = "Moderation"
)]
pub async fn welcome(ctx: Context<'_>, member: serenity::Member) -> Result<(), Error> {
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
