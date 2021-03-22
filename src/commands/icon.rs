use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    utils::MessageBuilder,
};
use tracing::error;

use crate::{
    data::{GuildIconStorage, ReqwestClientContainer},
    image_utils::{get_image, ImageType},
};

#[command]
#[only_in(guilds)]
#[sub_commands(set, get, list, add, del, clear)]
#[description("Icon management")]
#[num_args(0)]
pub async fn icon(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
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
#[description("Gets server icon")]
#[num_args(0)]
pub async fn get(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    if msg.guild_id.is_none() {
        error!("Message has no guild_id");
        return Ok(());
    }

    let guild_id = msg.guild_id.unwrap();
    let partial_guild = guild_id.to_partial_guild(&ctx.http).await.unwrap();
    match partial_guild.icon_url() {
        Some(icon) => {
            if let Err(why) = msg.channel_id.say(&ctx.http, icon).await {
                error!("Client error: {:?}", why);
            }
        }
        None => {
            if let Err(why) = msg.channel_id.say(&ctx.http, "No icon.").await {
                error!("Client error: {:?}", why);
            }
        }
    };

    Ok(())
}

#[command]
#[only_in(guilds)]
#[description("Sets server icon")]
#[num_args(1)]
pub async fn set(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
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
    let mut partial_guild = guild_id.to_partial_guild(&ctx.http).await.unwrap();

    let client = {
        ctx.data
            .read()
            .await
            .get::<ReqwestClientContainer>()
            .unwrap()
            .clone()
    };

    let icon = match get_image(&client, url, ImageType::GuildIcon).await {
        Ok(icon) => icon,
        Err(e) => {
            if let Err(why) = msg.reply(&ctx.http, format!("Error: {}", e)).await {
                error!("{}", why);
            };

            return Ok(());
        }
    };

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
    if msg.guild_id.is_none() {
        error!("Message has no guild_id");
        return Ok(());
    }

    let guild_id = msg.guild_id.unwrap();

    let storage_lock = {
        let data = ctx.data.read().await;
        data.get::<GuildIconStorage>().unwrap().clone()
    };

    let content = {
        let storage = storage_lock.read().await;

        let entries = match storage.get(&guild_id) {
            Some(entries) => entries,
            None => {
                if let Err(why) = msg.reply(&ctx.http, "No icons.").await {
                    error!("Client error: {:?}", why);
                }
                return Ok(());
            }
        };

        if entries.len() <= 0 {
            if let Err(why) = msg.reply(&ctx.http, "No icons.").await {
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
#[description("Adds server icon to storage")]
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
        data.get::<GuildIconStorage>().unwrap().clone()
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
#[description("Remove all server icons from storage")]
#[num_args(0)]
pub async fn clear(ctx: &Context, msg: &Message, _: Args) -> CommandResult {
    if msg.guild_id.is_none() {
        error!("Message has no guild_id");
        return Ok(());
    }

    let guild_id = msg.guild_id.unwrap();

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
