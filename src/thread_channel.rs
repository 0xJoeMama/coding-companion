use serenity::http::CacheHttp;
use serenity::{client::Context, model::channel::Message};

use crate::bot::Bot;

pub async fn create_thread(bot: &Bot, ctx: Context, msg: Message) -> Option<()> {
    // TODO: Maybe cache channel IDs here?
    let channel = msg
        .channel(ctx.http())
        .await
        .ok()
        .and_then(|channel| channel.guild())
        .filter(|channel| bot.cfg.thread_channels.contains(channel.name()));

    if let Some(thread_channel) = channel {
        let thread_name = create_thread_name(bot, &msg);

        let thread = thread_channel
            .create_public_thread(ctx.http(), msg.id, |thread| thread.name(&thread_name))
            .await
            .ok()?;

        thread
            .send_message(ctx.http(), |msg| {
                msg.content(&bot.cfg.thread_creation_message)
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

fn create_thread_name(bot: &Bot, msg: &Message) -> String {
    // TODO: Maybe use a config for the default value here!
    let thread_name = bot.emoji_regex.replace_all(msg.content.trim(), "");
    if thread_name.is_empty() {
        format!("Message from {}", msg.author.name)
    } else {
        thread_name.as_ref().to_owned()
    }
}
