use serenity::{
    async_trait,
    client::{Context, EventHandler},
    model::{
        channel::{Message, Reaction},
        gateway::{Activity, ActivityType, Ready},
        prelude::{
            command::CommandOptionType,
            interaction::{Interaction, InteractionType},
        },
    },
};

use crate::{
    commands::{CommandInstance, CommandParseError},
    config::Config,
    reaction, thread_channel, util,
};

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
        if msg.author.id != ctx.cache.current_user().id {
            thread_channel::create_thread(ctx, msg).await;
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction.kind() {
            InteractionType::ApplicationCommand => {
                let cmd = interaction.application_command().unwrap();
                let res: Result<_, CommandParseError> = CommandInstance::try_from(cmd);

                let res = match res {
                    Ok(handler) => handler.handle(ctx).await,
                    Err(error) => match error {
                        CommandParseError::NonExistingCommand => Ok(()),
                        CommandParseError::InvalidArgs { cmd } => {
                            util::invalid_arguments(&ctx, &cmd).await
                        }
                    },
                };

                if let Err(error) = res {
                    eprintln!("Could not handle command: {error}");
                }
            }
            kind => {
                eprintln!("Interaction of kind {kind:?} was queued. Ignoring it...")
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
                .set_application_commands(&ctx, |cmds| {
                    cmds
                        .create_application_command(|cmd| cmd
                            .name("ping")
                            .description("Ping")
                        )
                        .create_application_command(|cmd| cmd
                            .name("role_message")
                            .description("Create a message that will have all the reaction role reaction added to it.")
                        )
                        .create_application_command(|cmd| cmd
                            .name("purge")
                            .description("Delete a certain amount of messages from a channel.")
                            .create_option(|opt| opt
                                .name("count")
                                .description("Amount of messages to delete.")
                                .min_int_value(2)
                                .max_int_value(100)
                                .kind(CommandOptionType::Integer)
                            )
                        )
                        .create_application_command(|cmd| cmd
                            .name("say")
                            .description("Make the bot say something!")
                            .create_option(|opt| opt
                                .name("message")
                                .description("The message to bot should print.")
                                .max_length(4096)
                                .kind(CommandOptionType::String)
                            )
                        )
                        .create_application_command(|cmd| cmd
                            .name("lock")
                            .description("Lock the current text channel. This makes it visible but disallows non-admins from typing.")
                        )
                        .create_application_command(|cmd| cmd
                            .name("unlock")
                            .description("Unlocks the current text channel. This makes it public!")
                        )
                        .create_application_command(|cmd| cmd
                            .name("tldr")
                            .description("Give a tldr.")
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
