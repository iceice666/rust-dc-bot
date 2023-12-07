use poise::serenity_prelude::Member;

use crate::{Context, Result};

/// response with a pong!
#[poise::command(slash_command)]
pub async fn ping(ctx: Context<'_>, #[description = "target"] member: Option<Member>) -> Result {
    ctx.send(|b| {
        let target = match member {
            Some(v) => v.user.id,
            None => ctx.author().id,
        };

        b.content(format!("<@{}> Pong!", target)).reply(true)
    })
    .await?;
    Ok(())
}
