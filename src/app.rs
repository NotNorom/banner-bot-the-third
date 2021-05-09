use anyhow::{Context, Result};

use crate::data::*;
use crate::groups::*;
use crate::handler::*;
use crate::help::*;
use crate::hooks::*;

use std::{
    collections::HashSet,
    sync::{atomic::AtomicBool, Arc},
};

use dashmap::DashMap;
use serenity::{framework::StandardFramework, http::Http, Client};

/// Wrapper for the serenity code
pub struct App {
    http: Arc<Http>,
    serenity_client: serenity::Client,
    //reqwest_client: reqwest::Client,
}

impl App {
    /// Creates a new app
    pub fn new(
        http: Http,
        serenity_client: serenity::Client, /*, reqwest_client: reqwest::Client*/
    ) -> Self {
        Self {
            http: Arc::new(http),
            serenity_client,
            //reqwest_client,
        }
    }

    /// Get a reference to the app's http.
    pub fn http(&self) -> Arc<Http> {
        Arc::clone(&self.http)
    }

    /// Start the app
    pub async fn run(&mut self) -> Result<()> {
        self.serenity_client.start().await.map_err(|e| e.into())
    }
}

/// Creates an app instance with given `token`
pub async fn create_app(token: String) -> Result<App> {
    let http = Http::new_with_token(&token);

    let owners = {
        let info = http
            .get_current_application_info()
            .await
            .context("Could not get application info")?;

        let mut owners = HashSet::new();
        if let Some(team) = info.team {
            owners.insert(team.owner_user_id);
        } else {
            owners.insert(info.owner.id);
        }
        owners
    };
    let bot_id = http
        .get_current_user()
        .await
        .context("Could not get bot id")?
        .id;

    let framework = StandardFramework::new()
        .configure(|c| {
            c.with_whitespace(true)
                .on_mention(Some(bot_id))
                .prefix("b!")
                .owners(owners)
        })
        .before(before)
        .after(after)
        //.normal_message(normal_message)
        .help(&HELP)
        .group(&GENERAL_GROUP)
        .group(&STORAGE_GROUP);

    let serenity_client = Client::builder(&token)
        .event_handler(Handler {
            running: AtomicBool::new(false),
        })
        .framework(framework)
        .await
        .context("Failed to create serenity client")?;

    let guild_banner_storage = Arc::new(DashMap::new());
    let guild_icon_storage = Arc::new(DashMap::new());

    {
        let mut data = serenity_client.data.write().await;
        data.insert::<ShardManagerContainer>(serenity_client.shard_manager.clone());
        data.insert::<ReqwestClient>(reqwest::Client::new());
        data.insert::<GuildBannerStorage>(guild_banner_storage);
        data.insert::<GuildIconStorage>(guild_icon_storage);
    }

    let shard_manager = serenity_client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .context("Could not register ctrl+c handler")
            .unwrap();
        shard_manager.lock().await.shutdown_all().await;
    });

    Ok(App::new(http, serenity_client))
}
