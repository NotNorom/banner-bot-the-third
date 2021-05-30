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
#[description("Remove all banners/icons from storage")]
#[usage("clear <banner/icon>")]
#[num_args(1)]
pub async fn clear(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let guild_id = msg.guild_id.expect("This is a guild_only command");
    let image_type = args.parse::<DiscordImage>()?;

    let storage = {
        let data = ctx.data.read().await;
        match image_type {
            DiscordImage::GuildIcon => data.get::<GuildIconStorage>().unwrap().clone(),
            DiscordImage::GuildBanner => data.get::<GuildBannerStorage>().unwrap().clone(),
        }
    };

    let mut urls = storage.entry(guild_id).or_default();
    urls.clear();

    Ok(())
}
