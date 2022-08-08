use coding_companion::{config::Config, handler::Handler};

use serenity::{prelude::GatewayIntents, Client};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let token = dotenv::var("DISCORD_TOKEN")
        .expect("Could not locate an DISCORD_TOKEN environment variable!");

    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MESSAGE_REACTIONS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let config = Config::new("./config.json")?;

    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .type_map_insert::<Config>(config)
        .await
        .expect("Could not authorize the bot's client!");

    if let Err(error) = client.start().await {
        println!("Couldn't start the bot's client: {error}");
    }

    Ok(())
}
