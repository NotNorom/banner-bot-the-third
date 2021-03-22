use crate::data::*;
use crate::groups::*;
use crate::handler::*;
use crate::help::*;
use crate::hooks::*;

use std::{
    collections::{HashMap, HashSet},
    sync::{atomic::AtomicBool, Arc},
};

use serenity::{framework::StandardFramework, http::Http, prelude::RwLock, Client};

pub struct App {
    http: Arc<Http>,
    serenity_client: serenity::Client,
    //reqwest_client: reqwest::Client,
}

impl App {
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

    pub async fn run(&mut self) {
        if let Err(why) = self.serenity_client.start().await {
            println!("Client error: {:?}", why);
        }
    }
}

pub async fn create_app(token: String) -> App {
    let http = Http::new_with_token(&token);
    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else {
                owners.insert(info.owner.id);
            }
            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("Could not access the bot id: {:?}", why),
            }
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| {
            c.with_whitespace(true)
                .on_mention(Some(bot_id))
                .prefix("b!")
                .owners(owners)
        })
        .before(before)
        //.normal_message(normal_message)
        .help(&HELP)
        .group(&GENERAL_GROUP)
        .group(&ICON_GROUP)
        .group(&BANNER_GROUP);

    let serenity_client = Client::builder(&token)
        .event_handler(Handler {
            running: AtomicBool::new(false),
        })
        .framework(framework)
        .await
        .expect("Err creating client");

    let guild_banner_storage = Arc::new(RwLock::new(HashMap::new()));
    let guild_icon_storage = Arc::new(RwLock::new(HashMap::new()));

    {
        let mut data = serenity_client.data.write().await;
        data.insert::<ShardManagerContainer>(serenity_client.shard_manager.clone());
        data.insert::<ReqwestClientContainer>(reqwest::Client::new());
        data.insert::<GuildBannerStorage>(guild_banner_storage);
        data.insert::<GuildIconStorage>(guild_icon_storage);
    }

    let shard_manager = serenity_client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Could not register ctrl+c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    App::new(http, serenity_client)
}
