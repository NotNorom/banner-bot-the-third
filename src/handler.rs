use std::sync::atomic::AtomicBool;

use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::prelude::*,
};

use tracing::info;

pub struct Handler {
    pub running: AtomicBool,
}

#[async_trait]
impl EventHandler for Handler {
    async fn cache_ready(&self, _ctx: Context, _guilds: Vec<GuildId>) {
        info!("Cache ready.");
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        use serenity::model::gateway::Activity;
        use serenity::model::user::OnlineStatus;

        info!("Connected as {}", ready.user.name);

        let activity = Activity::listening("your commands, senpai~");
        let status = OnlineStatus::Online;

        ctx.set_presence(Some(activity), status).await;
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed");
    }
}
