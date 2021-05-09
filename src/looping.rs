use std::time::Duration;

use futures::Future;
use serenity::{client::Context, model::channel::Message, prelude::TypeMapKey};
use tokio::{task::JoinHandle, time::interval};
use tracing::error;

use crate::{
    data::{GuildTimerStorage, InternalTimerStorage},
    errors::BannerBotError,
};

/// Loop `task` every `minutes`.
/// If `task` returns Error stop looping.
pub async fn loop_with_ctx_and_msg<F, T, S>(
    ctx: Context,
    msg: Message,
    minutes: u64,
    task: F,
) -> Result<(), BannerBotError>
where
    F: Fn(Context, Message) -> T,
    F: Send + Sync + 'static,
    T: Future<Output = Result<(), BannerBotError>> + Send,
    S: TypeMapKey<Value = JoinHandle<()>>,
{
    let local_ctx = ctx.clone();
    let local_msg = msg.clone();

    let handle = tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(/* 60 */ minutes));
        loop {
            interval.tick().await;
            if let Err(e) = task(local_ctx.clone(), local_msg.clone()).await {
                error!("{:?}", e);
                break;
            };
        }
    });

    let guild_id = msg.guild_id.expect("This is a guild_only command");

    let storage = {
        let mut data = ctx.data.write().await;
        data.get_mut::<GuildTimerStorage>()
            .ok_or_else(|| BannerBotError::StorageNotInitialized)?
            .clone()
    };

    let mut handles = storage.entry(guild_id).or_default();

    handles
        .entry::<S>()
        .and_modify(|handle| handle.abort())
        .or_insert(handle);

    Ok(())
}

#[allow(dead_code)]
/// Loop `task` every `minutes` minutes.
/// If `task` returns Error stop looping.
///
pub async fn loop_with_ctx<F, T, S>(
    ctx: Context,
    name: String,
    minutes: u64,
    task: F,
) -> Result<(), BannerBotError>
where
    F: Fn(Context) -> T,
    F: Send + Sync + 'static,
    T: Future<Output = Result<(), BannerBotError>> + Send,
    S: TypeMapKey<Value = JoinHandle<()>>,
{
    let local_ctx = ctx.clone();

    let handle = tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(/* 60 */ minutes));
        loop {
            interval.tick().await;
            if let Err(e) = task(local_ctx.clone()).await {
                error!("{:?}", e);
                break;
            };
        }
    });

    let storage = {
        let mut data = ctx.data.write().await;
        data.get_mut::<InternalTimerStorage>()
            .ok_or_else(|| BannerBotError::StorageNotInitialized)?
            .clone()
    };

    let mut handles = storage.entry(name).or_default();

    handles
        .entry::<S>()
        .and_modify(|handle| handle.abort())
        .or_insert(handle);

    Ok(())
}
