use serenity::{
    async_trait,
    client::{Context, EventHandler},
    http::CacheHttp,
    model::{
        channel::{Message, Reaction},
        gateway::{Activity, ActivityType, Ready},
        prelude::interaction::{application_command::CommandDataOptionValue, Interaction},
    },
};

use crate::{commands::Commands, config::Config, reaction, thread_channel, util};

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
        if let Some(cmd) = interaction.application_command() {
            let opt = &cmd.data.options;

            let res = match cmd.data.name.as_str() {
                "ping" => Commands::Ping.handle(ctx, cmd).await,
                "role_message" => Commands::CreateRoleMessage.handle(ctx, cmd).await,
                "purge" => {
                    let cnt = opt.get(0).and_then(|it| it.resolved.as_ref());

                    if let Some(CommandDataOptionValue::Integer(cnt)) = cnt {
                        Commands::Purge {
                            msg_cnt: *cnt as u64,
                        }
                        .handle(ctx, cmd)
                        .await
                    } else {
                        util::invalid_arguments(ctx, cmd).await
                    }
                }
                _ => Err(serenity::Error::Other("Could not find the target command")),
            };

            if let Err(error) = res {
                println!("Error handling command: {error}");
            }
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
            .expect("Could not find Config in TypeMap")
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
            println!("Setting commands for guild with id {}", guild.id);
            if let Err(error) = guild
                .id
                .set_application_commands(ctx.http(), |cmds| {
                    cmds
                        .create_application_command(|cmd| cmd
                            .name("ping")
                            .description("Ping")
                        )
                        .create_application_command(|cmd| cmd
                            .name("role_message")
                            .description("Create a message that will have all the reaction role reaction added to it.")
                        )
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
