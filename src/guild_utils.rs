use rand::prelude::SliceRandom;
use serenity::{client::Context, model::channel::Message};

use crate::{
    data::{GuildBannerStorage, GuildIconStorage, ReqwestClient},
    errors::BannerBotError,
    image_utils::{get_image, DiscordImage},
};

pub async fn set_random_guild_image(
    ctx: Context,
    msg: Message,
    image_type: DiscordImage,
) -> Result<(), BannerBotError> {
    let guild_id = msg.guild_id.expect("This is a guild_only command");
    let mut partial_guild = guild_id.to_partial_guild(&ctx.http).await?;

    let storage = {
        let data = ctx.data.read().await;
        match image_type {
            DiscordImage::GuildIcon => data.get::<GuildIconStorage>(),
            DiscordImage::GuildBanner => data.get::<GuildBannerStorage>(),
        }
        .ok_or_else(|| BannerBotError::StorageNotInitialized)?
        .clone()
    };

    let urls = storage.entry(guild_id).or_default();

    let reqwest_client = {
        let data = ctx.data.read().await;
        data.get::<ReqwestClient>()
            .ok_or_else(|| BannerBotError::StorageNotInitialized)?
            .clone()
    };

    // TODO: Add some fallback logic for when a url can not be downloaded

    if urls.len() == 0 {
        return Err(BannerBotError::StorageEmpty);
    }

    let url = {
        let mut rng = rand::thread_rng();
        urls.choose(&mut rng).expect("There should be urls")
    };

    let image = get_image(&reqwest_client, url.as_str(), image_type)
        .await
        .unwrap();

    Ok(partial_guild
        .edit(&ctx.http, |g| match image_type {
            DiscordImage::GuildIcon => g.icon(Some(&image)),
            DiscordImage::GuildBanner => g.banner(Some(&image)),
        })
        .await?)
}
