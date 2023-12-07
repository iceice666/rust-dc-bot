use crate::{Data, Error, Result};
use poise::serenity_prelude as serenity;
use poise::Event;

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &Event<'_>,
    framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result {
    match event {
        Event::Ready { data_about_bot } => {
            let commands = &framework.options().commands;
            let create_commands = poise::builtins::create_application_commands(commands);
            serenity::Command::set_global_application_commands(ctx, |b| {
                *b = create_commands; // replace the given builder with the one prepared by poise
                b
            })
            .await?;
        }
        _ => {}
    }

    Ok(())
}

