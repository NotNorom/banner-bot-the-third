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
#[description("Removes banner/icon from storage")]
#[usage("del <banner/icon> <index>")]
#[num_args(2)]
pub async fn del(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = msg.guild_id.expect("This is a guild_only command");
    let image_type = args.single::<DiscordImage>()?;
    let idx = args.single::<usize>()?;

    let storage = {
        let data = ctx.data.read().await;
        match image_type {
            DiscordImage::GuildIcon => data.get::<GuildIconStorage>().unwrap().clone(),
            DiscordImage::GuildBanner => data.get::<GuildBannerStorage>().unwrap().clone(),
        }
    };

    let mut urls = storage.entry(guild_id).or_default();
    if idx >= urls.len() {
        return Err(format!("Entry {} does not exist", idx).into());
    }
    urls.remove(idx);

    Ok(())
}
