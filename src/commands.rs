use serenity::{
    client::Context,
    model::{
        channel::ReactionType,
        prelude::interaction::{
            application_command::ApplicationCommandInteraction, InteractionResponseType,
        },
        Permissions,
    },
    utils::Colour,
    Error,
};
use tokio::try_join;

use crate::{config::Config, util};

const ROLE_MESSAGE: &str = include_str!("../res/role_message.md");

pub enum Commands {
    Ping,
    CreateRoleMessage,
    Purge { msg_cnt: u64 },
    Say { msg: String },
    Lock,
    Unlock,
}

impl Commands {
    pub async fn handle(
        &self,
        ctx: Context,
        cmd: ApplicationCommandInteraction,
    ) -> Result<(), serenity::Error> {
        match self {
            Self::Ping => {
                cmd.create_interaction_response(&ctx, |res| {
                    res.kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|data| {
                            data.embed(|embed| embed.description("Pong!").colour(Colour::PURPLE))
                        })
                })
                .await?;
            }
            Self::CreateRoleMessage => {
                let guild_id = {
                    if cmd.guild_id.is_none() {
                        return Err(Error::Other("Not in a guild!"));
                    }

                    cmd.guild_id.unwrap()
                };

                let data = ctx.data.read().await;
                let cfg = data.get::<Config>().expect("Could not find Config data!");

                if cmd.channel_id != cfg.reaction_roles.channel {
                    cmd.create_interaction_response(&ctx, |res| res.
                        interaction_response_data(|data| data.
                            ephemeral(true).embed(|embed| embed
                                .title("Wrong Channel!")
                                .colour(Colour::RED)
                                .description("You tried to create a reaction role message in the wrong channel!")
                            )
                        )
                    ).await?;
                    return Err(Error::Other("Invalid channel id"));
                }

                cmd.create_interaction_response(&ctx, |res| {
                    res.interaction_response_data(|data| {
                        data.embed(|embed| {
                            embed
                                .title("Reaction Roles.")
                                .description(ROLE_MESSAGE)
                                .colour(Colour::PURPLE)
                        })
                    })
                })
                .await?;

                if let Ok(res) = cmd.get_interaction_response(&ctx).await {
                    for reaction in guild_id
                        .emojis(&ctx)
                        .await
                        .iter()
                        .flatten()
                        .filter(|it| cfg.reaction_roles.role_map.contains_key(&it.name))
                        .map(|emote| ReactionType::Custom {
                            animated: emote.animated,
                            name: Some(emote.name.clone()),
                            id: emote.id,
                        })
                    {
                        res.react(&ctx, reaction).await?;
                    }
                }
            }
            Self::Purge { msg_cnt } => {
                let channel = cmd.channel_id.to_channel(&ctx).await?;
                if let Some(channel) = channel.guild() {
                    let msgs = channel
                        .messages(&ctx, |msgs| msgs.limit(*msg_cnt))
                        .await?
                        .iter()
                        .map(|msg| msg.id)
                        .collect::<Vec<_>>();

                    channel.delete_messages(&ctx, &msgs).await?;

                    cmd.create_interaction_response(&ctx, |res| {
                        res.interaction_response_data(|data| {
                            data.embed(|embed| {
                                embed
                                    .description(format!(
                                        "**{}** purged {} messages!",
                                        cmd.user.name,
                                        &msgs.len()
                                    ))
                                    .colour(Colour::BLUE)
                            })
                        })
                    })
                    .await?
                }
            }
            Self::Say { msg } => {
                let bot_name = ctx.cache.current_user().name;
                let icon = ctx.cache.current_user().avatar_url().unwrap_or_default();

                cmd.create_interaction_response(ctx, |res| {
                    res.kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|data| {
                            data.embed(|embed| {
                                embed
                                    .author(|author| author.name(bot_name).icon_url(icon))
                                    .colour(Colour::PURPLE)
                                    .description(msg)
                            })
                        })
                })
                .await?
            }
            Self::Lock => {
                let perms = Permissions::empty()
                    | Permissions::VIEW_CHANNEL
                    | Permissions::READ_MESSAGE_HISTORY;

                try_join!(
                    util::apply_permissions(&ctx, cmd.channel_id, perms, "@everyone"),
                    cmd.create_interaction_response(&ctx, |res| {
                        res.interaction_response_data(|data| {
                            data.embed(|embed| {
                                embed
                                    .description(format!(
                                        "**{}** locked this channel!",
                                        cmd.user.name
                                    ))
                                    .colour(Colour::PURPLE)
                            })
                        })
                    })
                )?
                .0
            }
            Self::Unlock => {
                let perms = Permissions::empty()
                    | Permissions::VIEW_CHANNEL
                    | Permissions::READ_MESSAGE_HISTORY
                    | Permissions::CREATE_PUBLIC_THREADS
                    | Permissions::SEND_MESSAGES
                    | Permissions::SEND_MESSAGES_IN_THREADS
                    | Permissions::EMBED_LINKS
                    | Permissions::ATTACH_FILES
                    | Permissions::ADD_REACTIONS
                    | Permissions::USE_EXTERNAL_EMOJIS
                    | Permissions::USE_EXTERNAL_STICKERS;

                try_join!(
                    util::apply_permissions(&ctx, cmd.channel_id, perms, "@everyone"),
                    cmd.create_interaction_response(&ctx, |res| {
                        res.interaction_response_data(|data| {
                            data.embed(|embed| {
                                embed
                                    .description(format!(
                                        "**{}** unlocked this channel!",
                                        cmd.user.name
                                    ))
                                    .colour(Colour::PURPLE)
                            })
                        })
                    })
                )?
                .0
            }
        }

        Ok(())
    }
}
