use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use tracing::error;

use crate::image_utils::{get_image, ImageType};

#[command]
#[only_in(guilds)]
#[sub_commands(set, get)]
#[description("Icon management")]
#[num_args(0)]
pub async fn icon(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    if let Err(why) = msg
        .channel_id
        .say(&ctx.http, "Please use a subcommand")
        .await
    {
        error!("Client error: {:?}", why);
    };
    Ok(())
}

#[command]
#[only_in(guilds)]
#[description("Get server icon")]
#[num_args(0)]
pub async fn get(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    if msg.guild_id.is_none() {
        error!("Message has no guild_id");
        return Ok(());
    }

    let guild_id = msg.guild_id.unwrap();
    let partial_guild = guild_id.to_partial_guild(&ctx.http).await.unwrap();
    let icon = partial_guild.icon_url().unwrap();

    if let Err(why) = msg.channel_id.say(&ctx.http, icon).await {
        error!("Client error: {:?}", why);
    };
    Ok(())
}

#[command]
#[only_in(guilds)]
#[description("Set server icon")]
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
            };

            return Ok(());
        }
    };

    let guild_id = msg.guild_id.unwrap();
    let mut partial_guild = guild_id.to_partial_guild(&ctx.http).await.unwrap();

    let client = {
        ctx.data
            .read()
            .await
            .get::<crate::data::ReqwestClientContainer>()
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

    if let Err(why) = msg.react(&ctx.http, '👌').await {
        error!("Client error: {:?}", why);
    };
    Ok(())
}
