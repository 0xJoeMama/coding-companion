use serenity::{
    client::Context,
    http::CacheHttp,
    model::prelude::interaction::{
        application_command::ApplicationCommandInteraction, InteractionResponseType,
    },
    utils::Colour,
    Error,
};

pub async fn invalid_arguments(
    ctx: Context,
    cmd: ApplicationCommandInteraction,
) -> Result<(), Error> {
    cmd.create_interaction_response(ctx.http(), |res| {
        res.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|msg| {
                msg.embed(|embed| {
                    embed
                        .title("Invalid arguments!")
                        .description(
                            "Please make sure you provided the correct arguments for the command!",
                        )
                        .colour(Colour::RED)
                })
            })
    })
    .await
}
