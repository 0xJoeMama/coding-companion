use regex::Regex;
use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        channel::{Message, Reaction},
        gateway::{Activity, ActivityType, Ready},
    },
};

use crate::reaction;
use crate::{config::Config, thread_channel};

#[derive(Debug)]
pub struct Bot {
    pub cfg: Config,
    pub token: String,
    pub path: String,
    pub emoji_regex: Regex,
}

impl Bot {
    pub fn new(cfg_path: &str) -> std::io::Result<Bot> {
        let token = dotenv::var("DISCORD_TOKEN")
            .expect("Could not locate an DISCORD_TOKEN environment variable!");

        let cfg = Config::new(cfg_path)?;

        Ok(Bot {
            cfg,
            token,
            path: cfg_path.to_owned(),
            emoji_regex: Regex::new(r"<:.+:[0-9]+>").unwrap(), // TODO: Make this configurable?!
        })
    }
}

#[async_trait]
impl EventHandler for Bot {
    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        reaction::add_role(self, ctx, reaction).await;
    }

    async fn reaction_remove(&self, ctx: Context, reaction: Reaction) {
        reaction::remove_role(self, ctx, reaction).await;
    }

    async fn message(&self, ctx: Context, msg: Message) {
        thread_channel::create_thread(self, ctx, msg).await;
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!(
            "Initialized bot successfully with name '{}' and raw ID {}",
            ready.user.name, ready.user.id
        );
        let status = &self.cfg.status;

        let activity = match status.activity {
            ActivityType::Listening => Some(Activity::listening(&status.message)),
            ActivityType::Playing => Some(Activity::playing(&status.message)),
            ActivityType::Watching => Some(Activity::watching(&status.message)),
            _ => None,
        };

        ctx.set_activity(activity.unwrap_or_else(|| Activity::listening("everyone's requests!")))
            .await;
    }
}
