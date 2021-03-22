use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    utils::MessageBuilder,
};
use tracing::error;

use crate::{
    data::{GuildBannerStorage, ReqwestClientContainer},
    image_utils::{get_image, ImageType},
};

#[command]
#[only_in(guilds)]
#[sub_commands(set, get, list, add, del, clear)]
#[description("Banner management")]
#[num_args(0)]
pub async fn banner(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    if let Err(why) = msg
        .channel_id
        .say(&ctx.http, "Please use a subcommand")
        .await
    {
        error!("Client error: {:?}", why);
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
#[description("Gets server banner")]
#[num_args(0)]
pub async fn get(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    if msg.guild_id.is_none() {
        error!("Message has no guild_id");
        return Ok(());
    }

    let guild_id = msg.guild_id.unwrap();
    let partial_guild = guild_id.to_partial_guild(&ctx.http).await.unwrap();
    let banner = partial_guild.banner.as_ref().map(|banner| {
        format!(
            "https://cdn.discordapp.com/banners/{}/{}.webp",
            guild_id, banner
        )
    });

    match banner {
        Some(banner) => {
            if let Err(why) = msg.channel_id.say(&ctx.http, banner).await {
                error!("Client error: {:?}", why);
            }
        }
        None => {
            if let Err(why) = msg.channel_id.say(&ctx.http, "No banner").await {
                error!("Client error: {:?}", why);
            }
        }
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
#[description("Sets server banner")]
#[num_args(1)]
pub async fn set(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if msg.guild_id.is_none() {
        error!("Message has no guild_id");
        return Ok(());
    }

    let url = match args.single::<reqwest::Url>() {
        Ok(url) => url,
        Err(e) => {
            if let Err(why) = msg.reply(&ctx.http, format!("{}", e)).await {
                error!("{}", why);
            }

            return Ok(());
        }
    };

    let guild_id = msg.guild_id.unwrap();
    let mut partial_guild = guild_id.to_partial_guild(&ctx.http).await.unwrap();

    let client = {
        ctx.data
            .read()
            .await
            .get::<ReqwestClientContainer>()
            .unwrap()
            .clone()
    };

    let banner = match get_image(&client, url, ImageType::GuildBanner).await {
        Ok(banner) => banner,
        Err(e) => {
            if let Err(why) = msg.reply(&ctx.http, format!("{}", e)).await {
                error!("{}", why);
            };

            return Ok(());
        }
    };

    partial_guild
        .edit(&ctx.http, |g| g.banner(Some(&banner)))
        .await?;

    Ok(())
}

#[command]
#[only_in(guilds)]
#[description("Lists all known banners")]
#[num_args(0)]
pub async fn list(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    if msg.guild_id.is_none() {
        error!("Message has no guild_id");
        return Ok(());
    }

    let guild_id = msg.guild_id.unwrap();

    let storage_lock = {
        let data = ctx.data.read().await;
        data.get::<GuildBannerStorage>().unwrap().clone()
    };

    let content = {
        let storage = storage_lock.read().await;

        let entries = match storage.get(&guild_id) {
            Some(entries) => entries,
            None => {
                if let Err(why) = msg.reply(&ctx.http, "No banners.").await {
                    error!("Client error: {:?}", why);
                }
                return Ok(());
            }
        };

        if entries.len() <= 0 {
            if let Err(why) = msg.reply(&ctx.http, "No banners.").await {
                error!("Client error: {:?}", why);
            }
            return Ok(());
        }

        entries
            .iter()
            .enumerate()
            .fold(&mut MessageBuilder::new(), |builder, (idx, url)| {
                builder.push_line(format!("{:>3}: {}", idx, url.as_str()))
            })
            .build()
    };

    if let Err(why) = msg.reply(&ctx.http, content).await {
        error!("Client error: {:?}", why);
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
#[description("Adds server banner to storage")]
#[num_args(1)]
pub async fn add(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if msg.guild_id.is_none() {
        error!("Message has no guild_id");
        return Ok(());
    }

    let url = match args.single::<reqwest::Url>() {
        Ok(url) => url,
        Err(e) => {
            if let Err(why) = msg.reply(&ctx.http, format!("Error: {}", e)).await {
                error!("{}", why);
            }

            return Ok(());
        }
    };

    let guild_id = msg.guild_id.unwrap();

    let storage_lock = {
        let data = ctx.data.read().await;
        data.get::<GuildBannerStorage>().unwrap().clone()
    };

    {
        let mut storage = storage_lock.write().await;
        storage.entry(guild_id).or_default().push(url);
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
#[description("Removes server banner from storage")]
#[num_args(1)]
pub async fn del(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    if msg.guild_id.is_none() {
        error!("Message has no guild_id");
        return Ok(());
    }

    let idx = match args.single::<usize>() {
        Ok(idx) => idx,
        Err(e) => {
            if let Err(why) = msg.reply(&ctx.http, format!("Error: {}", e)).await {
                error!("{}", why);
            }

            return Ok(());
        }
    };

    let guild_id = msg.guild_id.unwrap();

    let storage_lock = {
        let data = ctx.data.read().await;
        data.get::<GuildBannerStorage>().unwrap().clone()
    };

    {
        let mut storage = storage_lock.write().await;
        let urls = storage.entry(guild_id).or_default();
        if idx >= urls.len() {
            if let Err(why) = msg
                .reply(
                    &ctx.http,
                    "Error: The url you want to remove does not exist",
                )
                .await
            {
                error!("Client error: {:?}", why);
            }
        } else {
            urls.remove(idx);
        }
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
#[description("Remove all server banners from storage")]
#[num_args(0)]
pub async fn clear(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    if msg.guild_id.is_none() {
        error!("Message has no guild_id");
        return Ok(());
    }

    let guild_id = msg.guild_id.unwrap();

    let storage_lock = {
        let data = ctx.data.read().await;
        data.get::<GuildBannerStorage>().unwrap().clone()
    };

    {
        let mut storage = storage_lock.write().await;
        let urls = storage.entry(guild_id).or_default();
        urls.clear();
    }

    Ok(())
}
