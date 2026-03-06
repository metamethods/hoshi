use twilight_model::application::command::{Command, CommandType};
use twilight_util::builder::command::CommandBuilder;

use crate::{
    commands::{ALL_CONTEXTS, ALL_INTEGRATIONS},
    error::BotResult,
    interaction::ApplicationCommandInteraction,
    resolver::ApplicationCommandInteractionMessageDataResolver,
    response::BotResponse,
};

pub fn schema() -> Command {
    CommandBuilder::new("echo", "", CommandType::Message)
        .integration_types(ALL_INTEGRATIONS)
        .contexts(ALL_CONTEXTS)
        .build()
}

pub async fn command(
    interaction: &mut ApplicationCommandInteraction<
        '_,
        '_,
        ApplicationCommandInteractionMessageDataResolver,
    >,
) -> BotResult<Option<BotResponse>> {
    interaction
        .reply(BotResponse::Content(
            interaction.data_resolver.message.content.clone(),
        ))
        .await?;

    Ok(None)
}
