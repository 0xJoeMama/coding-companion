use serenity::{
    async_trait,
    client::{Context, EventHandler},
    http::CacheHttp,
    model::{
        channel::{Message, Reaction},
        gateway::{Activity, ActivityType, Ready},
        prelude::interaction::{Interaction, InteractionType},
    },
};

use crate::{commands::Commands, config::Config, reaction, thread_channel};

#[derive(Debug)]
pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        reaction::add_role(ctx, reaction).await;
    }

    async fn reaction_remove(&self, ctx: Context, reaction: Reaction) {
        reaction::remove_role(ctx, reaction).await;
    }

    async fn message(&self, ctx: Context, msg: Message) {
        thread_channel::create_thread(ctx, msg).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction.kind() {
            InteractionType::ApplicationCommand => {
                println!("Received command");
                // unwrapping works since we just checked if it's a command
                let cmd = interaction.application_command().unwrap();

                let res = match cmd.data.name.as_str() {
                    "ping" => Commands::Ping.handle(ctx, cmd).await,
                    _ => Err(serenity::Error::Other("Could not find the target command")),
                };

                if let Err(error) = res {
                    println!("Error handling command: {error}");
                }
            }
            _ => {}
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!(
            "Initialized bot successfully with name '{}' and raw ID {}",
            ready.user.name, ready.user.id
        );
        let data = ctx.data.read().await;
        let status = &data
            .get::<Config>()
            .expect("Could not find config in TypeMap")
            .status;

        let activity = match status.activity {
            ActivityType::Listening => Some(Activity::listening(&status.message)),
            ActivityType::Playing => Some(Activity::playing(&status.message)),
            ActivityType::Watching => Some(Activity::watching(&status.message)),
            _ => None,
        };

        ctx.set_activity(activity.unwrap_or_else(|| Activity::listening("everyone's requests!")))
            .await;

        for guild in ready.guilds {
            println!("Setting commands for {}", guild.id);
            if let Err(error) = guild
                .id
                .set_application_commands(ctx.http(), |cmds| {
                    cmds.create_application_command(|cmd| cmd.name("ping").description("Ping"))
                })
                .await
            {
                println!(
                    "Could not set application commands for guild {}: {:?}",
                    guild.id, error
                );
            }
        }
    }
}
