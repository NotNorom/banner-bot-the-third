use std::sync::Arc;

use dashmap::DashMap;
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

pub struct ReqwestClient;

impl TypeMapKey for ReqwestClient {
    type Value = Client;
}

pub struct GuildIconStorage;

impl TypeMapKey for GuildIconStorage {
    type Value = Arc<DashMap<GuildId, Vec<reqwest::Url>>>;
}

pub struct GuildBannerStorage;

impl TypeMapKey for GuildBannerStorage {
    type Value = Arc<DashMap<GuildId, Vec<reqwest::Url>>>;
}
