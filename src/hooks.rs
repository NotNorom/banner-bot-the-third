use serenity::{
    client::Context,
    framework::standard::{macros::hook, CommandResult, DispatchError},
    model::channel::Message,
    utils::MessageBuilder,
};
use tracing::{error, info, warn};

#[hook]
pub async fn before(_ctx: &Context, msg: &Message, command_name: &str) -> bool {
    info!(
        "before: Got command '{}' by user '{}'",
        command_name, msg.author.name
    );
    true
}

#[hook]
pub async fn normal_message(_ctx: &Context, msg: &Message) {
    info!("{}", msg.content);
}

#[hook]
pub async fn after(
    ctx: &Context,
    msg: &Message,
    command_name: &str,
    command_result: CommandResult,
) {
    if let Err(e) = command_result {
        error!("{:#?}", e);

        let mut builder = MessageBuilder::new();
        builder.push_codeblock_safe(format!("{:#?}", e), None);

        if let Some(err_source) = e.source() {
            builder.push_codeblock_safe(format!("{:#?}", err_source), None);
        }

        let content = builder.build();

        if let Err(why) = msg.reply(&ctx.http, content).await {
            error!("Client error: {:?}", why);
        };
        return;
    };
}

#[hook]
pub async fn unknown_command(ctx: &Context, msg: &Message, unknown_command_name: &str) {
    if let Err(why) = msg.reply(&ctx.http, "Unkown command. Try the help.").await {
        error!("{}", why);
    }
    warn!("Could not find command named '{}'", unknown_command_name);
}

#[hook]
pub async fn dispatch_error(ctx: &Context, msg: &Message, error: DispatchError) {
    match error {
        DispatchError::NotEnoughArguments { min, given } => {
            let s = if min == 1 {
                format!("Need {} argument, but only got {}.", min, given)
            } else {
                format!("Need {} arguments, but only got {}.", min, given)
            };

            let _ = msg.reply(&ctx, &s).await;
        }
        DispatchError::TooManyArguments { max, given } => {
            let s = format!("Max arguments allowed is {}, but got {}.", max, given);

            let _ = msg.reply(&ctx, &s).await;
        }
        DispatchError::OnlyForGuilds => {
            let s = "Command can only be used in guilds/servers";

            let _ = msg.reply(&ctx, s).await;
        }
        DispatchError::CheckFailed(reason_msg, reason) => {
            let s = format!("Check failed: {}.\n{}", reason_msg, reason);

            let _ = msg.reply(&ctx, &s).await;
        }
        _ => warn!("Unhandled dispatch error."),
    }
}
