use regex::Regex;
use serenity::utils::Color;
use serenity::{client::Context, model::channel::Message};

use crate::config::Config;

pub async fn create_thread(ctx: Context, msg: Message) -> Option<()> {
    let data = ctx.data.read().await;
    let cfg = data.get::<Config>()?;

    // TODO: Maybe cache channel IDs here?
    let channel = msg
        .channel(&ctx)
        .await
        .ok()
        .and_then(|channel| channel.guild())
        .filter(|channel| cfg.thread_channels.channels.contains(channel.name()));

    if let Some(thread_channel) = channel {
        let thread_name = create_thread_name(&msg);

        let thread = thread_channel
            .create_public_thread(&ctx, msg.id, |thread| thread.name(&thread_name))
            .await
            .ok()?;

        thread
            .send_message(&ctx, |msg| {
                msg.embed(|embed| {
                    embed
                        .color(Color::BLUE)
                        .description(&cfg.thread_channels.creation_message)
                })
            })
            .await
            .ok()?;

        println!(
            "Created thread '{}' for message {} in channel '{}'!",
            thread_name,
            msg.id,
            thread_channel.name()
        )
    }

    None
}

fn create_thread_name(msg: &Message) -> String {
    // TODO: Maybe make this cached? It was cached but switching to type map required we stopped caching it.
    let emoji_regex: Regex = Regex::new(r"<:.+:[0-9]+>").unwrap();
    // TODO: Maybe use a config for the default value here!
    let thread_name = emoji_regex.replace_all(msg.content.trim(), "");
    if thread_name.is_empty() {
        format!("Message from {}", msg.author.name)
    } else {
        thread_name.as_ref().to_owned()
    }
}
