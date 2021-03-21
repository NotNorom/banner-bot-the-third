use serenity::{client::Context, framework::standard::macros::hook, model::channel::Message};
use tracing::{error, info, warn};

#[hook]
pub async fn before(_ctx: &Context, msg: &Message, command_name: &str) -> bool {
    info!(
        "before_0: Got command '{}' by user '{}'",
        command_name, msg.author.name
    );
    true
}

#[hook]
pub async fn normal_message(_ctx: &Context, msg: &Message) {
    info!("{}", msg.content);
}

#[hook]
async fn unknown_command(ctx: &Context, msg: &Message, unknown_command_name: &str) {
    if let Err(why) = msg.reply(&ctx.http, "Unkown command. Try the help.").await {
        error!("{}", why);
    }
    warn!("Could not find command named '{}'", unknown_command_name);
}
