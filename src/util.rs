use serenity::{
    client::Context,
    model::{
        channel::{PermissionOverwrite, PermissionOverwriteType},
        id::ChannelId,
        prelude::interaction::{
            application_command::ApplicationCommandInteraction, InteractionResponseType,
        },
        Permissions,
    },
    utils::Colour,
    Error,
};

pub async fn invalid_arguments(
    ctx: &Context,
    cmd: &ApplicationCommandInteraction,
) -> Result<(), Error> {
    cmd.create_interaction_response(&ctx, |res| {
        res.kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|data| {
                data.embed(|embed| {
                    embed
                        .title("Invalid arguments!")
                        .description(
                            "Please make sure you provided the correct arguments for the command!",
                        )
                        .colour(Colour::RED)
                })
                .ephemeral(true)
            })
    })
    .await
}

pub async fn apply_permissions(
    ctx: &Context,
    channel_id: ChannelId,
    perms: Permissions,
    role: &str,
) -> Result<(), Error> {
    let channel = channel_id
        .to_channel(&ctx)
        .await
        .and_then(|it| it.guild().ok_or(Error::Other("Not a guild channel")))?;

    let guild = channel.guild(ctx);

    if let Some(guild) = guild {
        let everyone = guild
            .role_by_name(role)
            .ok_or(Error::Other("Could not find a requested role"))?;

        let overwrite = PermissionOverwrite {
            allow: perms,
            deny: !perms,
            kind: PermissionOverwriteType::Role(everyone.id),
        };

        channel.create_permission(&ctx, &overwrite).await?;
    };

    Ok(())
}
