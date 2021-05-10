use std::{collections::HashSet, sync::Arc};

use dashmap::DashMap;
use reqwest::Client;
use serenity::{
    client::bridge::gateway::ShardManager,
    model::id::{GuildId, RoleId},
    prelude::{Mutex, TypeMap, TypeMapKey},
};
use tokio::task::JoinHandle;

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

pub struct GuildAllowedRolesStorage;

impl TypeMapKey for GuildAllowedRolesStorage {
    type Value = Arc<DashMap<GuildId, HashSet<RoleId>>>;
}

/*
    TIMERS
*/

pub struct GuildTimerStorage;

impl TypeMapKey for GuildTimerStorage {
    type Value = Arc<DashMap<GuildId, TypeMap>>;
}

pub struct GuildBannerTimer;
impl TypeMapKey for GuildBannerTimer {
    type Value = JoinHandle<()>;
}

pub struct GuildIconTimer;
impl TypeMapKey for GuildIconTimer {
    type Value = JoinHandle<()>;
}



pub struct InternalTimerStorage;

impl TypeMapKey for InternalTimerStorage {
    type Value = Arc<DashMap<String, TypeMap>>;
}
