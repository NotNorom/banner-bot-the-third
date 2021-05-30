use futures::future::join_all;

use serenity::{
    client::Context,
    framework::standard::{macros::check, Args, CommandOptions, Reason},
    model::channel::Message,
};

use crate::{data, errors::BannerBotError};

#[check]
pub async fn member_has_allowed_role_or_is_admin(
    ctx: &Context,
    msg: &Message,
    _args: &mut Args,
    _cmd_options: &CommandOptions,
) -> Result<(), Reason> {
    let guild_id = msg
        .guild_id
        .ok_or_else(|| Reason::Log("Guild only check".into()))?;

    // see if member is an admin
    if msg
        .member(ctx)
        .await
        .map_err(|e| Reason::Log(e.to_string()))?
        .permissions(ctx)
        .await
        .map_err(|e| Reason::Log(e.to_string()))?
        .administrator()
    {
        return Ok(());
    }

    let storage = ctx
        .data
        .read()
        .await
        .get::<data::GuildAllowedRolesStorage>()
        .ok_or_else(|| BannerBotError::StorageNotInitialized)?
        .clone();

    let roles = storage
        .get(&guild_id)
        .ok_or_else(|| BannerBotError::StorageEmpty)?;

    // this is fucking ridiculous
    let role_checks = join_all(
        roles
            .iter()
            .map(|role_id| msg.author.has_role(ctx, guild_id, role_id)),
    )
    .await;

    match role_checks.into_iter().filter_map(Result::ok).any(|x| x) {
        true => Ok(()),
        false => Err(Reason::User("Missing role".into())),
    }
}

#[check]
pub async fn minimum_duration(
    _: &Context,
    _: &Message,
    args: &mut Args,
    _cmd_options: &CommandOptions,
) -> Result<(), Reason> {
    args.advance();
    let minutes = args
        .single::<u64>()
        .map_err(|e| Reason::Log(e.to_string()))?;
    if minutes < 30 {
        return Err(Reason::User("Duration must at least be 30 minutes".into()));
    }
    Ok(())
}
