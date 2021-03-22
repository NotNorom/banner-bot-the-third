use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    utils::MessageBuilder,
};

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
    msg.channel_id
        .say(&ctx.http, "Please use a subcommand")
        .await?;

    Ok(())
}

#[command]
#[only_in(guilds)]
#[description("Gets server banner")]
#[num_args(0)]
pub async fn get(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => return Err("Not a guild".into()),
    };

    let partial_guild = guild_id.to_partial_guild(&ctx.http).await?;
    // @note: wait for serenity library to add GuildId.banner_url() method
    let banner = partial_guild.banner.as_ref().map(|banner| {
        format!(
            "https://cdn.discordapp.com/banners/{}/{}.webp",
            guild_id, banner
        )
    });

    match banner {
        Some(banner) => msg.channel_id.say(&ctx.http, banner).await?,
        None => return Err("No banner".into()),
    };

    Ok(())
}

#[command]
#[only_in(guilds)]
#[description("Sets server banner")]
#[num_args(1)]
pub async fn set(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => return Err("Not a guild".into()),
    };
    let mut partial_guild = guild_id.to_partial_guild(&ctx.http).await?;

    let url = args.single::<reqwest::Url>()?;

    let client = {
        let data = ctx.data.read().await;
        data.get::<ReqwestClientContainer>().unwrap().clone()
    };

    let banner = get_image(&client, url, ImageType::GuildBanner).await?;

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
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => return Err("Not a guild".into()),
    };

    let storage_lock = {
        let data = ctx.data.read().await;
        data.get::<GuildBannerStorage>().unwrap().clone()
    };

    let content = {
        let storage = storage_lock.read().await;

        let entries = match storage.get(&guild_id) {
            Some(entries) => entries,
            None => return Err("No banners".into()),
        };

        if entries.len() <= 0 {
            return Err("No banners".into());
        }

        entries
            .iter()
            .enumerate()
            .fold(&mut MessageBuilder::new(), |builder, (idx, url)| {
                builder.push_line(format!("{:>3}: {}", idx, url.as_str()))
            })
            .build()
    };

    msg.reply(&ctx.http, content).await?;

    Ok(())
}

#[command]
#[only_in(guilds)]
#[description("Adds server banner to storage")]
#[num_args(1)]
pub async fn add(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => return Err("Not a guild".into()),
    };

    let url = args.single::<reqwest::Url>()?;

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
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => return Err("Not a guild".into()),
    };

    let idx = args.single::<usize>()?;

    let storage_lock = {
        let data = ctx.data.read().await;
        data.get::<GuildBannerStorage>().unwrap().clone()
    };

    {
        let mut storage = storage_lock.write().await;
        let urls = storage.entry(guild_id).or_default();
        if idx >= urls.len() {
            return Err(format!("Url at position {} does not exist", idx).into());
        }
        urls.remove(idx);
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
#[description("Remove all server banners from storage")]
#[num_args(0)]
pub async fn clear(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => return Err("Not a guild".into()),
    };

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
