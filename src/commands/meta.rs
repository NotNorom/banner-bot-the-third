use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};
use tracing::error;

use crate::data::ShardManagerContainer;


#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.channel_id.say(&ctx.http, "pong").await {
        error!("Client error: {:?}", why);
    };
    Ok(())
}

#[command]
#[owners_only]
#[description("Shut down bot :(")]
#[max_args(0)]
pub async fn shutdown(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;

    if let Some(manager) = data.get::<ShardManagerContainer>() {
        if let Err(why) = msg.channel_id.say(&ctx.http, "Shutting down shard...").await {
            error!("Client error: {:?}", why);
        };
        
        manager.lock().await.shutdown_all().await;
    } else {
        if let Err(why) = msg.reply(ctx, "There was a problem getting the shard manager").await {
            error!("Client error: {:?}", why);
        }

        return Ok(());
    }

    Ok(())
}