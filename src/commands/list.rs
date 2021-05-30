use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
    utils::MessageBuilder,
};

use crate::{
    data::{GuildBannerStorage, GuildIconStorage},
    image_utils::DiscordImage,
};

#[command]
#[only_in(guilds)]
#[description("Lists all known banners/icons")]
#[min_args(1)]
pub async fn list(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let guild_id = msg.guild_id.expect("This is a guild_only command");
    let image_type = args.parse::<DiscordImage>()?;

    let storage = {
        let data = ctx.data.read().await;
        match image_type {
            DiscordImage::GuildIcon => data.get::<GuildIconStorage>().unwrap().clone(),
            DiscordImage::GuildBanner => data.get::<GuildBannerStorage>().unwrap().clone(),
        }
    };

    let content = {
        let entries = storage.get(&guild_id).ok_or("No icon")?;

        if entries.len() <= 0 {
            return Err("No entries".into());
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
