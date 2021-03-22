use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::data::ShardManagerContainer;

#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx.http, "pong").await?;
    Ok(())
}

#[command]
#[owners_only]
#[description("Shut down bot :(")]
#[max_args(0)]
pub async fn shutdown(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;

    let manager = data
        .get::<ShardManagerContainer>()
        .ok_or("Could not get shard manager")?;

    msg.reply(&ctx.http, "Shutting down... 😢").await?;
    manager.lock().await.shutdown_all().await;

    Ok(())
}
