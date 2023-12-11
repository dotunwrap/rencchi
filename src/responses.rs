use crate::{Context, Error};
use poise::serenity_prelude as serenity;

pub async fn success(ctx: Context<'_>, msg: &str) -> Result<(), Error> {
    ctx.say(msg).await?;
    Ok(())
}

pub async fn failure(ctx: Context<'_>, msg: &str) -> Result<(), Error> {
    ctx.reply(msg).await?;
    Ok(())
}

pub async fn not_staff(ctx: Context<'_>) -> Result<(), Error> {
    failure(
        ctx,
        "Error: You do not have permission to use this command.",
    )
    .await
}

pub async fn paginate(
    ctx: Context<'static>,
    embeds: Vec<serenity::CreateEmbed>,
) -> Result<(), Error> {
    let ctx_id = ctx.id();
    let prev_button_id = format!("{}_prev", ctx_id);
    let next_button_id = format!("{}_next", ctx_id);

    let reply = {
        let components = serenity::CreateActionRow::Buttons(vec![
            serenity::CreateButton::new(&prev_button_id).emoji("⬅".chars().next().unwrap()),
            serenity::CreateButton::new(&next_button_id).emoji("➡".chars().next().unwrap()),
        ]);

        poise::CreateReply::default()
            .embed(embeds[0].clone())
            .components(vec![components])
    };

    ctx.send(reply).await?;

    let mut current_page = 0;

    while let Some(press) = serenity::collector::ComponentInteractionCollector::new(ctx)
        .filter(move |press| {
            press.user.id == ctx.author().id
                && press.data.custom_id.starts_with(&ctx_id.to_string())
        })
        .timeout(std::time::Duration::from_secs(180))
        .await
    {
        match press.data.custom_id.as_str().to_owned() {
            next_button_id => {
                current_page += 1;
                if current_page >= embeds.len() {
                    current_page = 0;
                }
            }
            prev_button_id => {
                current_page = current_page.checked_sub(1).unwrap_or(embeds.len() - 1);
            }
            _ => (),
        }

        press
            .create_response(
                ctx.serenity_context(),
                serenity::CreateInteractionResponse::UpdateMessage(
                    serenity::CreateInteractionResponseMessage::new()
                        .embed(embeds[current_page].clone()),
                ),
            )
            .await?;
    }

    Ok(())
}
