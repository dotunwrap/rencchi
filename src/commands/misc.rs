use crate::{Context, Error};

#[poise::command(prefix_command, slash_command, category = "Misc")]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Specific comand to get help with"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            extra_text_at_bottom: "\
            Run /help <command> for more info on a command.",
            ..Default::default()
        },
    )
    .await?;

    Ok(())
}
