use std::sync::Arc;

use twilight_model::{
    application::interaction::{Interaction as EventInteraction, InteractionData, InteractionType},
    channel::message::MessageFlags,
};
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::{
    components, context::BotContext, error::BotResult, interaction::ApplicationInteraction,
    response::BotResponse,
};

mod handlers;

async fn handle_interaction(
    application_interaction: &mut ApplicationInteraction<'_>,
    context: Arc<BotContext>,
) -> BotResult<Option<BotResponse>> {
    let event_kind = application_interaction.event_interaction.kind;
    let event_data = application_interaction.event_interaction.data.clone();

    match (event_kind, event_data) {
        (
            InteractionType::ApplicationCommand,
            Some(InteractionData::ApplicationCommand(command_data)),
        ) => {
            handlers::application_command::handler(application_interaction, command_data, context)
                .await
        }
        _ => Ok(Some(BotResponse::Error(
            tl!(
                application_interaction,
                "errors.unhandled",
                x = event_kind.kind()
            )
            .into(),
        ))),
    }
}

pub async fn event(event_interaction: EventInteraction, context: Arc<BotContext>) -> BotResult<()> {
    let mut application_interaction =
        ApplicationInteraction::new(event_interaction, context.interaction());

    let bot_response_data_option =
        match handle_interaction(&mut application_interaction, context.clone()).await {
            Ok(response_data) => response_data,
            Err(error) => Some(BotResponse::Error(error.to_string())),
        };

    match bot_response_data_option {
        Some(bot_response_data) => {
            let response_data = match bot_response_data {
                BotResponse::ResponseData(response_data) => response_data,
                BotResponse::Content(content) => InteractionResponseDataBuilder::new()
                    .content(content)
                    .build(),
                BotResponse::Error(error_string) => InteractionResponseDataBuilder::new()
                    .components([components::error::component(
                        error_string,
                        application_interaction.event_interaction.id,
                        application_interaction.locale(),
                    )])
                    .flags(MessageFlags::IS_COMPONENTS_V2.union(MessageFlags::EPHEMERAL))
                    .build(),
            };

            if application_interaction.is_deferred {
                application_interaction
                    .update_response_from_response_data(response_data)
                    .await?;
            } else if application_interaction.has_replied {
                application_interaction
                    .followup_from_response_data(response_data)
                    .await?;
            } else {
                application_interaction
                    .reply(BotResponse::ResponseData(response_data))
                    .await?;
            }
        }
        None => (),
    };

    Ok(())
}
