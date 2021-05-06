use std::time::Duration;

use rand::prelude::SliceRandom;
use serenity::{client::Context, model::id::GuildId};
use tokio::time::sleep;
use tracing::error;

use crate::{
    data::{GuildBannerStorage, GuildIconStorage, ReqwestClient},
    image_utils::{get_image, ImageType},
};

pub async fn shuffle(ctx: Context, guild_id: GuildId, image_type: ImageType, interval: Duration) {
    let mut partial_guild = guild_id.to_partial_guild(&ctx.http).await.unwrap();

    loop {
        let storage = {
            let data = ctx.data.read().await;
            match image_type {
                ImageType::GuildIcon => data.get::<GuildIconStorage>().unwrap().clone(),
                ImageType::GuildBanner => data.get::<GuildBannerStorage>().unwrap().clone(),
            }
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
                let url = urls.choose(&mut rng);
                match url {
                    Some(url) => url,
                    None => {
                        error!("no icons :(");
                        return;
                    }
                }
            };

            let image = get_image(&reqwest_client, url.as_str(), image_type)
                .await
                .unwrap();

            if let Err(e) = partial_guild
                .edit(&ctx.http, |g| match image_type {
                    ImageType::GuildIcon => g.icon(Some(&image)),
                    ImageType::GuildBanner => g.banner(Some(&image)),
                })
                .await
            {
                error!("{}", e);
            }
        };

        sleep(interval).await;
    }
}
