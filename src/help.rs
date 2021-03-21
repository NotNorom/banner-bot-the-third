use std::collections::HashSet;

use serenity::{
    client::Context,
    framework::standard::{help_commands, Args, CommandGroup, CommandResult, HelpOptions},
    model::{channel::Message, id::UserId},
};

#[serenity::framework::standard::macros::help]
#[max_levenshtein_distance(3)]
#[indention_prefix = "+"]
#[wrong_channel = "Strike"]
pub async fn help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}
