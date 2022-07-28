use serenity::{
    client::Context,
    http::CacheHttp,
    model::channel::{Reaction, ReactionType},
};

use crate::bot::Bot;

pub async fn add_role(bot: &Bot, ctx: Context, reaction: Reaction) -> Option<()> {
    if reaction.channel_id != bot.cfg.reaction_role_channel {
        return None;
    }

    if let ReactionType::Custom {
        animated: _,
        id: _,
        name: Some(emoji_name),
    } = &reaction.emoji
    {
        let role = bot.cfg.get_role(&emoji_name)?;
        let guild = reaction.guild_id?.to_guild_cached(&ctx)?;
        let mut member = guild
            .member(ctx.http(), reaction.user_id.unwrap())
            .await
            .ok()?;

        let role = guild.role_by_name(role)?;

        if member.add_role(ctx.http(), role.id).await.is_ok() {
            println!(
                "Added reaction role '{}' to user '{}'.",
                role.name,
                member.display_name()
            );
        }
        ()
    }

    None
}

pub async fn remove_role(bot: &Bot, ctx: Context, reaction: Reaction) -> Option<()> {
    if reaction.channel_id != bot.cfg.reaction_role_channel {
        return None;
    }

    if let ReactionType::Custom {
        animated: _,
        id: _,
        name: Some(emoji_name),
    } = &reaction.emoji
    {
        let role = bot.cfg.get_role(&emoji_name)?;
        let guild = reaction.guild_id?.to_guild_cached(&ctx)?;
        let mut member = guild
            .member(ctx.http(), reaction.user_id.unwrap())
            .await
            .ok()?;

        let role = guild.role_by_name(role)?;

        if member.remove_role(ctx.http(), role.id).await.is_ok() {
            println!(
                "Removed reaction role '{}' from '{}'.",
                role.name,
                member.display_name()
            );
        }
        ()
    }

    None
}
