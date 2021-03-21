use std::{sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
}, time::Duration};

use serenity::{async_trait, client::{Context, EventHandler}, http::CacheHttp, model::prelude::*};
use tokio::time::sleep;
use tracing::{info, error};

pub struct Handler {
    pub running: AtomicBool,
}

#[async_trait]
impl EventHandler for Handler {
    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        let ctx = Arc::new(ctx);

        if !self.running.load(Ordering::Relaxed) {
            let ctx1 = Arc::clone(&ctx);

            tokio::spawn(async move {
                let channel = ChannelId(710630746372702213);
                let mut counter = 0usize;
                loop {
                    if let Err(why) = channel.say(ctx1.http(), format!("Alive: {}", counter), ).await {
                        error!("{:?}", why);
                    }
                    counter += 1;
                    sleep(Duration::from_secs(30)).await;
                }
            });
        }

        self.running.swap(true, Ordering::Relaxed);
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
