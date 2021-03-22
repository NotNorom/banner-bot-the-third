use serenity::{client::Context, framework::standard::{CommandResult, macros::hook}, model::channel::Message};
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
pub async fn after(ctx: &Context, msg: &Message, _command_name: &str, command_result: CommandResult) {
    match command_result {
        Ok(_) => {
            if let Err(why) = msg.react(&ctx.http, '👌').await {
                error!("Client error: {:?}", why);
            };
        }
        Err(e) => {
            error!("{}", e);

            if let Err(why) = msg.react(&ctx.http, '❌').await {
                error!("Client error: {:?}", why);
            };
        }
    };
}

#[hook]
async fn unknown_command(ctx: &Context, msg: &Message, unknown_command_name: &str) {
    if let Err(why) = msg.reply(&ctx.http, "Unkown command. Try the help.").await {
        error!("{}", why);
    }
    warn!("Could not find command named '{}'", unknown_command_name);
}
