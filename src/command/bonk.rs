use poise::serenity_prelude::Member;

use crate::{Context, Result};

/// response with a pong!
#[poise::command(slash_command)]
pub async fn bonk(
    ctx: Context<'_>,
    #[description = "target"] member: Member,
    #[description = "message"] message: Option<String>,
) -> Result {
    ctx.send(|b| {
        let target = member.user.id;

        let msg = match message {
            Some(v) => v,
            None => "".to_string(),
        };

        b.content(format!(
            "<@{}> <@{}> bonked you! \n{} ",
            target,
            ctx.author().id,
            msg
        ))
        .reply(true)
    })
    .await?;
    Ok(())
}
