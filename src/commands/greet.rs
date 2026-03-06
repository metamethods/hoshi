use twilight_model::application::command::{Command, CommandType};
use twilight_util::builder::command::CommandBuilder;

use crate::{
    commands::{ALL_CONTEXTS, ALL_INTEGRATIONS},
    error::BotResult,
    interaction::ApplicationCommandInteraction,
    resolver::ApplicationCommandInteractionUserDataResolver,
    response::BotResponse,
};

pub fn schema() -> Command {
    CommandBuilder::new("greet", "", CommandType::User)
        .integration_types(ALL_INTEGRATIONS)
        .contexts(ALL_CONTEXTS)
        .build()
}

pub async fn command(
    interaction: &mut ApplicationCommandInteraction<
        '_,
        '_,
        ApplicationCommandInteractionUserDataResolver,
    >,
) -> BotResult<Option<BotResponse>> {
    interaction
        .reply(BotResponse::Content(
            tl!(
                interaction,
                "commands.greet.response",
                user_id = interaction.data_resolver.user.id.to_string()
            )
            .into(),
        ))
        .await?;

    Ok(None)
}
