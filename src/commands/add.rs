use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

use crate::{
    data::{GuildBannerStorage, GuildIconStorage},
    image_utils::DiscordImage,
};

#[command]
#[only_in(guilds)]
#[description("Add banners/icons to storage")]
#[usage("add <banner/icon> <url> [url] [url] ...")]
#[min_args(2)]
pub async fn add(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = msg.guild_id.expect("This is a guild_only command");
    let image_type = args.parse::<DiscordImage>()?;
    args.advance();

    let mut urls: Vec<_> = args
        .iter::<reqwest::Url>()
        .filter_map(Result::ok)
        .map(|url| url.to_string())
        .collect();

    let storage = {
        let data = ctx.data.read().await;
        match image_type {
            DiscordImage::GuildIcon => data.get::<GuildIconStorage>().unwrap().clone(),
            DiscordImage::GuildBanner => data.get::<GuildBannerStorage>().unwrap().clone(),
        }
    };

    storage.entry(guild_id).or_default().append(&mut urls);

    Ok(())
}
