use serenity::{
    client::Context,
    framework::standard::{macros::command, Args, CommandResult},
    model::channel::Message,
};

use crate::{
    checks::*,
    data::{GuildBannerTimer, GuildIconTimer},
    image_utils::DiscordImage,
};

#[command]
#[only_in(guilds)]
#[checks(member_has_allowed_role_or_is_admin)]
#[checks(minimum_duration)]
#[description("Shuffle icons every few minutes. Minimum duration is 30 minutes")]
#[usage("shuffle <banner/icon> <minutes>")]
#[num_args(2)]
pub async fn shuffle(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let image_type = args.single::<DiscordImage>()?;
    let minutes = args.single::<u64>()?;

    match image_type {
        DiscordImage::GuildIcon => {
            crate::looping::loop_with_ctx_and_msg::<_, _, GuildIconTimer>(
                ctx.clone(),
                msg.clone(),
                minutes,
                move |ctx, msg| async move {
                    crate::guild_utils::set_random_guild_image(ctx, msg, image_type)
                        .await?;
                    Ok(())
                },
            )
            .await?
        }
        DiscordImage::GuildBanner => {
            crate::looping::loop_with_ctx_and_msg::<_, _, GuildBannerTimer>(
                ctx.clone(),
                msg.clone(),
                minutes,
                move |ctx, msg| async move {
                    crate::guild_utils::set_random_guild_image(ctx, msg, image_type)
                        .await?;
                    Ok(())
                },
            )
            .await?
        }
    };

    Ok(())
}
