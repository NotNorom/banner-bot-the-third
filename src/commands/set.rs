use reqwest::Url;
use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

use crate::{
    data::ReqwestClient,
    image_utils::{get_image, DiscordImage},
};

#[command]
#[only_in(guilds)]
#[description("Set things")]
#[usage("Set <banner|icon> <url>")]
#[min_args(1)]
pub async fn set(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = msg.guild_id.expect("This is a guild_only command");
    let mut partial_guild = guild_id.to_partial_guild(&ctx.http).await?;

    let image_type = args.parse::<DiscordImage>()?;
    args.advance();
    let url = args.parse::<Url>()?;

    let image = {
        let data = ctx.data.read().await;
        let client = data.get::<ReqwestClient>().unwrap().clone();
        get_image(&client, url, image_type).await?
    };

    partial_guild
        .edit(&ctx.http, |g| match image_type {
            DiscordImage::GuildIcon => g.icon(Some(&image)),
            DiscordImage::GuildBanner => g.banner(Some(&image)),
        })
        .await?;

    Ok(())
}
