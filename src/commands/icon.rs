use std::time::Duration;

use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    utils::MessageBuilder,
};

use crate::{
    data::{GuildIconStorage, ReqwestClient},
    image_utils::{get_image, ImageType},
};

#[command]
#[only_in(guilds)]
#[sub_commands(set, get, list, add, del, clear, shuffle)]
#[description("Icon management")]
#[num_args(0)]
pub async fn icon(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    msg.channel_id
        .say(&ctx.http, "Please use a subcommand")
        .await?;

    Ok(())
}

#[command]
#[only_in(guilds)]
#[description("Gets server icon")]
#[num_args(0)]
pub async fn get(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => return Err("Not a guild".into()),
    };
    let partial_guild = guild_id.to_partial_guild(&ctx.http).await.unwrap();

    match partial_guild.icon_url() {
        Some(icon) => msg.channel_id.say(&ctx.http, icon).await?,
        None => return Err("No icon".into()),
    };

    Ok(())
}

#[command]
#[only_in(guilds)]
#[description("Sets server icon")]
#[num_args(1)]
pub async fn set(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => return Err("Not a guild".into()),
    };
    let mut partial_guild = guild_id.to_partial_guild(&ctx.http).await.unwrap();

    let url = args.single::<reqwest::Url>()?;

    let client = {
        let data = ctx.data.read().await;
        data.get::<ReqwestClient>().unwrap().clone()
    };

    let icon = get_image(&client, url, ImageType::GuildIcon).await?;

    partial_guild
        .edit(&ctx.http, |g| g.icon(Some(&icon)))
        .await?;

    Ok(())
}

#[command]
#[only_in(guilds)]
#[description("Lists all known icons")]
#[num_args(0)]
pub async fn list(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => return Err("Not a guild".into()),
    };

    let storage_lock = {
        let data = ctx.data.read().await;
        data.get::<GuildIconStorage>().unwrap().clone()
    };

    let content = {
        let storage = storage_lock.read().await;

        let entries = match storage.get(&guild_id) {
            Some(entries) => entries,
            None => return Err("No icon".into()),
        };

        if entries.len() <= 0 {
            return Err("No icons".into());
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
#[description("Adds server icon to storage")]
#[num_args(1)]
pub async fn add(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => return Err("Not a guild".into()),
    };

    let url = args.single::<reqwest::Url>()?;

    let storage_lock = {
        let data = ctx.data.read().await;
        data.get::<GuildIconStorage>().unwrap().clone()
    };

    {
        let mut storage = storage_lock.write().await;
        storage.entry(guild_id).or_default().push(url);
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
#[description("Removes server icon from storage")]
#[num_args(1)]
pub async fn del(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => return Err("Not a guild".into()),
    };

    let idx = args.single::<usize>()?;

    let storage_lock = {
        let data = ctx.data.read().await;
        data.get::<GuildIconStorage>().unwrap().clone()
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
#[description("Remove all server icons from storage")]
#[num_args(0)]
pub async fn clear(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => return Err("Not a guild".into()),
    };

    let storage_lock = {
        let data = ctx.data.read().await;
        data.get::<GuildIconStorage>().unwrap().clone()
    };

    {
        let mut storage = storage_lock.write().await;
        let urls = storage.entry(guild_id).or_default();
        urls.clear();
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
#[description("Shuffle icons")]
#[num_args(1)]
pub async fn shuffle(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => return Err("Not a guild".into()),
    };

    let interval_minutes = args.single::<u64>()?;
    let interval = Duration::from_secs(60 * interval_minutes);

    let ctx1 = ctx.clone();
    let _ = tokio::spawn(async move {
        crate::timers::shuffle(ctx1, guild_id, ImageType::GuildIcon, interval).await;
    });

    Ok(())
}
