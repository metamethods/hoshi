use twilight_model::{
    application::command::{Command, CommandType},
    channel::message::MessageFlags,
};
use twilight_util::builder::{InteractionResponseDataBuilder, command::CommandBuilder};

use crate::{
    commands::{ALL_CONTEXTS, ALL_INTEGRATIONS},
    components,
    error::BotResult,
    interaction::ApplicationCommandInteraction,
    resolver::ApplicationCommandInteractionChatInputDataResolver,
    response::BotResponse,
};

pub fn schema() -> Command {
    CommandBuilder::new("ping", "ping", CommandType::ChatInput)
        .integration_types(ALL_INTEGRATIONS)
        .contexts(ALL_CONTEXTS)
        .build()
}

pub async fn command(
    interaction: &mut ApplicationCommandInteraction<
        '_,
        '_,
        ApplicationCommandInteractionChatInputDataResolver,
    >,
) -> BotResult<Option<BotResponse>> {
    let time = std::time::Instant::now();

    interaction.defer_reply(MessageFlags::empty()).await?;

    let one_way_trip_time = time.elapsed().as_millis();

    let _ = interaction.get_response().await; // obtain the original message (or try to)

    let round_trip_time = time.elapsed().as_millis();

    interaction
        .update_response_from_response_data(
            InteractionResponseDataBuilder::new()
                .components([components::ping::component(
                    round_trip_time,
                    one_way_trip_time,
                    interaction.locale(),
                )])
                .flags(MessageFlags::IS_COMPONENTS_V2)
                .build(),
        )
        .await?;

    Ok(None)
}
