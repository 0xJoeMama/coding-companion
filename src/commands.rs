use serenity::{
    client::Context,
    http::CacheHttp,
    model::{
        channel::ReactionType,
        prelude::interaction::{
            application_command::ApplicationCommandInteraction, InteractionResponseType,
        },
    },
    utils::Colour,
    Error,
};

use crate::config::Config;

const ROLE_MESSAGE: &str = include_str!("../res/role_message.md");

pub enum Commands {
    Ping,
    CreateRoleMessage,
    Purge { msg_cnt: u64 },
    Say { msg: String },
}

impl Commands {
    pub async fn handle(
        &self,
        ctx: Context,
        cmd: ApplicationCommandInteraction,
    ) -> Result<(), serenity::Error> {
        match self {
            Commands::Ping => {
                cmd.create_interaction_response(ctx.http(), |res| {
                    res.kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|data| {
                            data.embed(|embed| embed.description("Pong!").colour(Colour::PURPLE))
                        })
                })
                .await?;
            }
            Commands::CreateRoleMessage => {
                let guild_id = {
                    if cmd.guild_id.is_none() {
                        return Err(Error::Other("Not in a guild!"));
                    }

                    cmd.guild_id.unwrap()
                };

                let data = ctx.data.read().await;
                let cfg = data.get::<Config>().expect("Could not find Config data!");

                if cmd.channel_id != cfg.reaction_role_channel {
                    cmd.create_interaction_response(ctx.http(), |res| res
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|data| data
                        .ephemeral(true)
                        .embed(|embed| embed
                        .title("Wrong channel!")
                        .colour(Colour::RED)
                        .description("You tried to create a reaction role message in the wrong channel! You can only use the configured channel!"))))
                    .await?;
                    return Err(Error::Other("Invalid channel id"));
                }

                cmd.create_interaction_response(ctx.http(), |res| {
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

                if let Ok(res) = cmd.get_interaction_response(ctx.http()).await {
                    for reaction in guild_id
                        .emojis(ctx.http())
                        .await
                        .iter()
                        .flatten()
                        .filter(|it| cfg.reaction_roles.contains_key(&it.name))
                        .map(|emote| ReactionType::Custom {
                            animated: emote.animated,
                            name: Some(emote.name.clone()),
                            id: emote.id,
                        })
                    {
                        res.react(ctx.http(), reaction).await?;
                    }
                }
            }
            Commands::Purge { msg_cnt } => {
                let channel = cmd.channel_id.to_channel(ctx.http()).await?;
                if let Some(channel) = channel.guild() {
                    let msgs = channel
                        .messages(ctx.http(), |msgs| msgs.limit(*msg_cnt))
                        .await?
                        .iter()
                        .map(|msg| msg.id)
                        .collect::<Vec<_>>();

                    channel.delete_messages(ctx.http(), msgs).await?;
                }
            }
            Commands::Say { msg } => {
                let bot_name = ctx.cache.current_user().name;
                let icon = ctx
                    .cache
                    .current_user()
                    .avatar_url()
                    .unwrap_or_else(|| String::new());
                cmd.create_interaction_response(ctx.http(), |res| {
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
        }

        Ok(())
    }
}
