use crate::{responses, utils::staff, Context, Error};
use std::{thread, time::Duration};

/// Purges message(s) (staff only)
///
/// (SLASH | PREFIX) purge [amount] [purge-all]
/// [amount] is the number of messages to purge. It must be between 1 and 100. It defaults to 100 if not specified.
/// [purge-all] determines whether to purge all messages in the channel. It defaults to false if not specified.
/// If [purge-all] is true, [amount] is ignored.
#[poise::command(
    prefix_command,
    slash_command,
    check = "staff::staff_check",
    required_bot_permissions = "MANAGE_MESSAGES",
    category = "Moderation"
)]
pub async fn purge(
    ctx: Context<'_>,
    #[description = "Number of messages to purge"]
    #[min = 1]
    #[max = 100]
    amount: Option<usize>,
    #[rename = "purge-all"]
    #[description = "Purge all messages in the channel"]
    purge_all: Option<bool>,
) -> Result<(), Error> {
    let amount = amount.unwrap_or(100);
    let purge_all = purge_all.unwrap_or(false);
    let deleted_count = match purge_all {
        true => self::purge_all_messages(ctx, None).await.unwrap(),
        false => self::purge_messages(ctx, amount).await.unwrap(),
    };

    match deleted_count {
        1 => responses::success(ctx, "Purged 1 message.").await,
        _ => responses::success(ctx, &format!("Purged {} messages.", deleted_count)).await,
    }
}

async fn purge_all_messages(ctx: Context<'_>, count: Option<usize>) -> Result<usize, Error> {
    let mut count = count.unwrap_or(0);
    let messages = ctx
        .channel_id()
        .messages(&ctx, serenity::builder::GetMessages::new().limit(100))
        .await?;

    ctx.reply("Purge in progress...").await?;

    for message in &messages {
        count += 1;
        purge_message(ctx, message).await?;
        thread::sleep(Duration::from_millis(5000));
    }

    Ok(count)
}

async fn purge_messages(ctx: Context<'_>, mut amount: usize) -> Result<usize, Error> {
    if ctx.prefix() == "." {
        amount += 1;
    }

    let messages_to_delete = ctx
        .channel_id()
        .messages(
            &ctx,
            serenity::builder::GetMessages::new().limit(amount as u8),
        )
        .await?
        .into_iter()
        .take(amount);

    ctx.channel_id()
        .delete_messages(&ctx, messages_to_delete)
        .await?;

    Ok(if ctx.prefix() == "." {
        amount - 1
    } else {
        amount
    })
}

async fn purge_message(
    ctx: Context<'_>,
    message: &poise::serenity_prelude::Message,
) -> Result<(), Error> {
    message.delete(ctx).await?;
    Ok(())
}
