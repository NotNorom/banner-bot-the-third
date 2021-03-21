use crate::data::*;
use crate::groups::*;
use crate::handler::*;
use crate::help::*;
use crate::hooks::*;

use std::{
    collections::HashSet,
    env,
    sync::{atomic::AtomicBool, Arc},
    time::Duration,
};

use serenity::{
    framework::StandardFramework, http::Http, model::id::ChannelId, prelude::Mutex, Client,
};
use tokio::time::sleep;
use tracing::error;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

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
        .group(&ICON_GROUP);

    let serenity_client = Client::builder(&token)
        .event_handler(Handler {
            running: AtomicBool::new(false),
        })
        .framework(framework)
        .await
        .expect("Err creating client");

    let reqwest_client = reqwest::Client::new();

    {
        let mut data = serenity_client.data.write().await;
        data.insert::<ShardManagerContainer>(serenity_client.shard_manager.clone());
        data.insert::<ReqwestClientContainer>(reqwest_client.clone());
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

#[tokio::test]
async fn test_app_creation() {
    dotenv::dotenv().expect("Failed to load .env file");

    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .pretty()
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to start the logger");

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut app = create_app(token).await;
    let http = app.http();

    // tokio::spawn(async move {
    //     let channel = ChannelId(710630746372702213);
    //     loop {
    //         if let Err(why) = channel.say(http.clone(), "pusss").await {
    //             error!("{:?}", why);
    //         }
    //         sleep(Duration::from_secs(9)).await;
    //     }
    // });

    app.run().await;
}
