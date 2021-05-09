use std::env;

use banner_bot::app::create_app;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

#[tokio::main]
async fn main() {
    match dotenv::dotenv() {
        Ok(_) => {}
        Err(e) => eprintln!("Failed to load .env file. {:?}", e),
    };

    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .pretty()
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to start the logger");

    let token = match env::var("DISCORD_TOKEN") {
        Ok(token) => token,
        Err(e) => {
            eprintln!("Environment variable DISCORD_TOKEN is not set. {:?}", e);
            return;
        }
    };

    match create_app(token).await {
        Ok(mut app) => {
            if let Err(e) = app.run().await {
                eprintln!("{:?}", e);
            }
        }
        Err(e) => eprintln!("{:?}", e),
    }
}
