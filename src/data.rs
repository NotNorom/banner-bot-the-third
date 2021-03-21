use std::sync::Arc;

use reqwest::Client;
use serenity::{
    client::bridge::gateway::ShardManager,
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
