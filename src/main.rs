use coding_companion::bot::Bot;

use serenity::{framework::StandardFramework, prelude::GatewayIntents, Client};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let bot = Bot::new("./config.json")?;
    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MESSAGE_REACTIONS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let framework =
        StandardFramework::new().configure(|c| c.with_whitespace(true).prefix(&bot.cfg.prefix));

    let mut client = Client::builder(&bot.token, intents)
        .event_handler(bot) // TODO: Requires a static lifetime
        .framework(framework)
        .await
        .expect("Could not create a client for the bot!");

    if let Err(error) = client.start().await {
        println!("Couldn't start the bot: {error}");
    }

    Ok(())
}
