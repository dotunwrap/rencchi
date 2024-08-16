use crate::{Context, Error};

pub async fn success(ctx: Context<'_>, msg: &str) -> Result<(), Error> {
    ctx.say(msg).await?;
    Ok(())
}

pub async fn failure(ctx: Context<'_>, msg: &str) -> Result<(), Error> {
    ctx.reply(msg).await?;
    Ok(())
}

pub async fn invalid_permissions(ctx: Context<'_>) -> Result<(), Error> {
    failure(
        ctx,
        "You do not have permission to use this command. Please contact an administrator if you believe this is an error.",
    )
    .await
}
