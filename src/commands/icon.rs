use serenity::model::prelude::*;
use serenity::prelude::*;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    utils::MessageBuilder,
};

use crate::{
    checks::*,
    data::{GuildIconStorage, GuildIconTimer, ReqwestClient},
    image_utils::{get_image, DiscordImage},
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

    let icon = get_image(&client, url, DiscordImage::GuildIcon).await?;

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

    let storage = {
        let data = ctx.data.read().await;
        data.get::<GuildIconStorage>().unwrap().clone()
    };

    let content = {
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

    let storage = {
        let data = ctx.data.read().await;
        data.get::<GuildIconStorage>().unwrap().clone()
    };

    storage.entry(guild_id).or_default().push(url);

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

    let storage = {
        let data = ctx.data.read().await;
        data.get::<GuildIconStorage>().unwrap().clone()
    };

    let mut urls = storage.entry(guild_id).or_default();
    if idx >= urls.len() {
        return Err(format!("Url at position {} does not exist", idx).into());
    }
    urls.remove(idx);

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

    let storage = {
        let data = ctx.data.read().await;
        data.get::<GuildIconStorage>().unwrap().clone()
    };

    let mut urls = storage.entry(guild_id).or_default();
    urls.clear();

    Ok(())
}

#[command]
#[only_in(guilds)]
#[checks(member_has_allowed_role_or_is_admin)]
#[checks(minimum_duration)]
#[description("Shuffle icons every few minutes. Minimum duration is 30 minutes")]
#[usage("shuffle <minutes>")]
#[num_args(1)]
pub async fn shuffle(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let minutes = args.single::<u64>()?;

    crate::looping::loop_with_ctx_and_msg::<_, _, GuildIconTimer>(
        ctx.clone(),
        msg.clone(),
        minutes,
        move |ctx, msg| async move {
            crate::guild_utils::set_random_guild_image(ctx, msg, DiscordImage::GuildIcon).await?;
            Ok(())
        },
    )
    .await?;

    Ok(())
}
