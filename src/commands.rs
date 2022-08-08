use serenity::{
    client::Context,
    http::CacheHttp,
    model::prelude::interaction::{
        application_command::ApplicationCommandInteraction, InteractionResponseType,
    },
    utils::Color,
};

pub enum Commands {
    Ping,
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
                            data.embed(|embed| embed.description("Pong!").color(Color::PURPLE))
                        })
                })
                .await?;
            }
        }
        Ok(())
    }
}
