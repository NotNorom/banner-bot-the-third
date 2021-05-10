use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::{data::GuildAllowedRolesStorage, errors::BannerBotError};

#[command]
#[only_in(guilds)]
#[required_permissions(ADMINISTRATOR)]
#[description("List roles which are allowed to use this bot")]
#[min_args(1)]
pub async fn list_roles(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild_id = msg.guild_id.ok_or_else(|| "Not a guild")?;

    let storage = {
        let data = ctx.data.read().await;
        data.get::<GuildAllowedRolesStorage>()
            .ok_or_else(|| BannerBotError::StorageNotInitialized)?
            .clone()
    };

    let roles = storage.entry(guild_id).or_default();
    msg.reply(ctx, format!("{:?}", roles.value())).await?;

    Ok(())
}

#[command]
#[only_in(guilds)]
#[required_permissions(ADMINISTRATOR)]
#[description("Allow specific roles to use this bot")]
#[min_args(1)]
pub async fn allow_roles(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild_id = msg.guild_id.ok_or_else(|| "Not a guild")?;

    let storage = {
        let data = ctx.data.write().await;
        data.get::<GuildAllowedRolesStorage>()
            .ok_or_else(|| BannerBotError::StorageNotInitialized)?
            .clone()
    };

    let mut roles = storage.entry(guild_id).or_default();
    roles.extend(msg.mention_roles.iter());

    Ok(())
}

#[command]
#[only_in(guilds)]
#[required_permissions(ADMINISTRATOR)]
#[description("Clear roles to use this bot. Only administrators can use it then")]
#[min_args(1)]
pub async fn clear_roles(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild_id = msg.guild_id.ok_or_else(|| "Not a guild")?;

    let storage = {
        let data = ctx.data.write().await;
        data.get::<GuildAllowedRolesStorage>()
            .ok_or_else(|| BannerBotError::StorageNotInitialized)?
            .clone()
    };

    let mut roles = storage.entry(guild_id).or_default();
    roles.clear();

    Ok(())
}
