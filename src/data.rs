use std::{collections::HashMap, sync::Arc};

use reqwest::Client;
use serenity::{
    client::bridge::gateway::ShardManager,
    model::id::GuildId,
    prelude::{Mutex, RwLock, TypeMapKey},
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
    type Value = Arc<RwLock<HashMap<GuildId, Vec<reqwest::Url>>>>;
}

pub struct GuildBannerStorage;

impl TypeMapKey for GuildBannerStorage {
    type Value = Arc<RwLock<HashMap<GuildId, Vec<reqwest::Url>>>>;
}
