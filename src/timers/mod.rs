use std::time::Duration;

use rand::prelude::SliceRandom;
use serenity::{client::Context, model::id::GuildId};
use tokio::time::sleep;
use tracing::error;

use crate::{
    data::{GuildBannerStorage, GuildIconStorage, ReqwestClient},
    image_utils::{get_image, DiscordImage},
};

pub async fn shuffle(
    ctx: Context,
    guild_id: GuildId,
    image_type: DiscordImage,
    interval: Duration,
) {
    let mut partial_guild = guild_id.to_partial_guild(&ctx.http).await.unwrap();

    loop {
        let storage = {
            let data = ctx.data.read().await;
            match image_type {
                DiscordImage::GuildIcon => data.get::<GuildIconStorage>(),
                DiscordImage::GuildBanner => data.get::<GuildBannerStorage>(),
            }
            .unwrap()
            .clone()
        };

            let urls = match storage.get(&guild_id) {
                Some(urls) => urls,
                None => {
                    error!("no icons :(");
                    return;
                }
            };

            let reqwest_client = {
                let data = ctx.data.read().await;
                data.get::<ReqwestClient>().unwrap().clone()
            };
            
            // TODO: Add some fallback logic for when a url can not be downloaded

            let url = {
                let mut rng = rand::thread_rng();
            urls.choose(&mut rng)
            };

        match url {
            Some(url) => {
            let image = get_image(&reqwest_client, url.as_str(), image_type)
                .await
                .unwrap();
            if let Err(e) = partial_guild
                .edit(&ctx.http, |g| match image_type {
                        DiscordImage::GuildIcon => g.icon(Some(&image)),
                        DiscordImage::GuildBanner => g.banner(Some(&image)),
                })
                .await
            {
                error!("{}", e);
                    return ();
            }
            }
            None => return (),
        }

        sleep(interval).await;
    }
}
