use std::sync::Arc;

use twilight_model::application::{
    command::CommandType, interaction::application_command::CommandData,
};

use crate::{
    commands,
    context::BotContext,
    error::BotResult,
    interaction::{ApplicationCommandInteraction, ApplicationInteraction},
    match_command_arm,
    resolver::{
        ApplicationCommandInteractionChatInputDataResolver,
        ApplicationCommandInteractionMessageDataResolver,
        ApplicationCommandInteractionUserDataResolver,
    },
    response::BotResponse,
};

async fn handle_chat_input_command(
    interaction: &mut ApplicationCommandInteraction<
        '_,
        '_,
        ApplicationCommandInteractionChatInputDataResolver,
    >,
    context: Arc<BotContext>,
) -> BotResult<Option<BotResponse>> {
    let command_name = interaction.data_resolver.command.as_str();
    let subcommand_group_name = interaction
        .data_resolver
        .subcommand_group
        .as_ref()
        .map(|s| s.as_str());
    let subcommand_name = interaction
        .data_resolver
        .subcommand
        .as_ref()
        .map(|s| s.as_str());

    match (command_name, subcommand_group_name, subcommand_name) {
        match_command_arm!("ping") => commands::ping::command(interaction).await,
        match_command_arm!("roll") => commands::roll::command(interaction, context).await,

        #[cfg(debug_assertions)]
        match_command_arm!("error") => commands::debug::error::command().await,

        _ => Ok(Some(BotResponse::Error(
            tl!(interaction, "errors.unhandled", x = command_name).into(),
        ))),
    }
}

async fn handle_message_command(
    interaction: &mut ApplicationCommandInteraction<
        '_,
        '_,
        ApplicationCommandInteractionMessageDataResolver,
    >,
    _: Arc<BotContext>,
) -> BotResult<Option<BotResponse>> {
    let command_name = interaction.data_resolver.command.as_str();

    match command_name {
        "echo" => commands::echo::command(interaction).await,
        _ => Ok(Some(BotResponse::Error(
            tl!(interaction, "errors.unhandled", x = command_name).into(),
        ))),
    }
}

async fn handle_user_command(
    interaction: &mut ApplicationCommandInteraction<
        '_,
        '_,
        ApplicationCommandInteractionUserDataResolver,
    >,
    _: Arc<BotContext>,
) -> BotResult<Option<BotResponse>> {
    let command_name = interaction.data_resolver.command.as_str();

    match command_name {
        "greet" => commands::greet::command(interaction).await,
        _ => Ok(Some(BotResponse::Error(
            tl!(interaction, "errors.unhandled", x = command_name).into(),
        ))),
    }
}

pub async fn handler(
    application_interaction: &mut ApplicationInteraction<'_>,
    command_data: Box<CommandData>,
    context: Arc<BotContext>,
) -> BotResult<Option<BotResponse>> {
    match command_data.kind {
        CommandType::ChatInput => {
            let interaction_data_resolver =
                ApplicationCommandInteractionChatInputDataResolver::from_commmand_data(
                    command_data,
                );
            let interaction = &mut ApplicationCommandInteraction::new(
                application_interaction,
                interaction_data_resolver,
            );

            handle_chat_input_command(interaction, context).await
        }
        CommandType::Message => {
            let target_id = command_data
                .target_id
                .ok_or("message_data target_id's field is None")?;
            let target_messasge = command_data
                .resolved
                .as_ref()
                .and_then(|resolved| resolved.messages.get(&target_id.cast()))
                .ok_or("unable to fetch resolved target_message command_data")?
                .clone();

            let interaction_data_resolver = ApplicationCommandInteractionMessageDataResolver::new(
                command_data.name.clone(),
                target_messasge,
            );
            let interaction = &mut ApplicationCommandInteraction::new(
                application_interaction,
                interaction_data_resolver,
            );

            handle_message_command(interaction, context).await
        }
        CommandType::User => {
            let target_id = command_data
                .target_id
                .ok_or("message_data target_id's field is None")?;
            let target_user = command_data
                .resolved
                .as_ref()
                .and_then(|resolved| resolved.users.get(&target_id.cast()))
                .ok_or("unable to fetch resolved target_message command_data")?
                .clone();

            let interaction_data_resolver = ApplicationCommandInteractionUserDataResolver::new(
                command_data.name.clone(),
                target_user,
            );
            let interaction = &mut ApplicationCommandInteraction::new(
                application_interaction,
                interaction_data_resolver,
            );

            handle_user_command(interaction, context).await
        }
        _ => todo!(),
    }
}
