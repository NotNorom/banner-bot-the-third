use std::{collections::HashMap, sync::Arc};

use reqwest::Client;
use serenity::{
    client::bridge::gateway::ShardManager,
    model::id::GuildId,
    prelude::{Mutex, TypeMapKey},
};

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

pub struct ReqwestClientContainer;

impl TypeMapKey for ReqwestClientContainer {
    type Value = Client;
}

pub struct DelayQueueContainer;

impl TypeMapKey for DelayQueueContainer {
    type Value = Arc<Mutex<tokio_util::time::DelayQueue<i32>>>;
}

pub struct GuildIconStorage;

impl TypeMapKey for GuildIconStorage {
    type Value = Arc<Mutex<HashMap<GuildId, Vec<reqwest::Url>>>>;
}

pub struct GuildBannerStorage;

impl TypeMapKey for GuildBannerStorage {
    type Value = Arc<Mutex<HashMap<GuildId, Vec<reqwest::Url>>>>;
}
