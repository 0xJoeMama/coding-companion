use serenity::{
    async_trait, client::Context,
    model::prelude::interaction::application_command::ApplicationCommandInteraction,
};

#[async_trait]
trait Command<'a> {
    async fn handle(&self, _ctx: Context, _cmd: ApplicationCommandInteraction) {
        // _cmd.create_followup_message(_ctx.http(), |msg| msg.ephemeral);
    }

    fn name(&self) -> &'a str;
}

