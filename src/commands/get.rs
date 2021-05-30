use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

use crate::image_utils::DiscordImage;

#[command]
#[only_in(guilds)]
#[description("Get things")]
#[usage("get <banner|icon>")]
#[min_args(1)]
pub async fn get(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let guild_id = msg.guild_id.expect("This is a guild_only command");
    let partial_guild = guild_id.to_partial_guild(&ctx.http).await?;

    let image_type = args.parse::<DiscordImage>()?;

    let url = match image_type {
        DiscordImage::GuildIcon => partial_guild.icon_url().ok_or("No icon")?,
        DiscordImage::GuildBanner => {
            // @note: wait for serenity library to add PartialGuild.banner_url() method
            partial_guild
                .banner
                .as_ref()
                .map(|banner| {
                    format!(
                        "https://cdn.discordapp.com/banners/{}/{}.webp",
                        guild_id, banner
                    )
                })
                .ok_or("No banner")?
        }
    };
    msg.reply(&ctx, url).await?;

    Ok(())
}
