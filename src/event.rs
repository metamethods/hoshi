use twilight_gateway::Event;
use twilight_model::{
    application::interaction::{InteractionType, application_command::CommandOptionValue},
    channel::message::MessageFlags,
    http::interaction::InteractionResponseType,
};
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::{
    BotResult, autocompletes, commands,
    context::BotContext,
    interaction::{ApplicationCommandInteraction, ApplicationInteraction},
};

async fn handle_aplication_command(
    command_interaction: &mut ApplicationCommandInteraction<'_, '_>,
    context: &BotContext,
) -> BotResult<()> {
    match command_interaction.command_data.name.as_str() {
        "ping" => commands::ping::command(command_interaction).await,
        "ffmpeg" => commands::ffmpeg::command(command_interaction, context).await,
        "speechbubble" => commands::speechbubble::command(command_interaction, context).await,
        "pixelsort" => commands::pixelsort::command(command_interaction, context).await,
        _ => Err(t_application_interaction_err!(
            command_interaction.application_interaction,
            "error.event.interaction.command.unhandled",
            command_name = command_interaction.command_data.name
        )
        .into()),
    }
}

async fn handle_application_command_autocomplete(
    command_interaction: ApplicationCommandInteraction<'_, '_>,
) -> BotResult<()> {
    let focused_command_option = command_interaction
        .command_data
        .options
        .iter()
        .find(|option| matches!(option.value, CommandOptionValue::Focused(_, _)));

    if let Some(ref focused_command_option) = focused_command_option {
        let autocomplete_choices = match focused_command_option.value {
            CommandOptionValue::Focused(ref partial, _) => {
                match command_interaction.command_data.name.as_str() {
                    "ffmpeg" => {
                        autocompletes::ffmpeg::autocomplete(
                            focused_command_option.name.clone(),
                            partial.clone(),
                        )
                        .await?
                    }
                    _ => todo!(),
                }
            }
            _ => todo!(),
        };

        command_interaction
            .application_interaction
            .create_response(
                InteractionResponseType::ApplicationCommandAutocompleteResult,
                Some(
                    InteractionResponseDataBuilder::new()
                        .choices(autocomplete_choices)
                        .build(),
                ),
            )
            .await?;
    }

    Ok(())
}

async fn handle_interaction(
    application_interaction: &mut ApplicationInteraction<'_>,
    context: &BotContext,
) -> BotResult<()> {
    match application_interaction.event_interaction.kind {
        InteractionType::ApplicationCommand => {
            handle_aplication_command(
                &mut ApplicationCommandInteraction::new(application_interaction)?,
                context,
            )
            .await
        }
        InteractionType::ApplicationCommandAutocomplete => {
            handle_application_command_autocomplete(ApplicationCommandInteraction::new(
                application_interaction,
            )?)
            .await
        }
        _ => todo!(),
    }
}

pub async fn handle_event(event: Event, context: BotContext) -> BotResult<()> {
    match event {
        Event::Ready(ready) => {
            println!(
                "ready event received for {}#{}",
                ready.user.name,
                ready.user.discriminator()
            );
        }
        Event::InteractionCreate(interaction) => {
            let interaction_client = context.interaction();
            let mut application_interaction =
                ApplicationInteraction::new(interaction.0, &interaction_client);

            match handle_interaction(&mut application_interaction, &context).await {
                Err(error) => {
                    let content = &format!(
                        "{}\n```\n{error:?}\n```",
                        t_application_interaction!(
                            application_interaction,
                            "error.event.interaction.generic"
                        )
                    );

                    if application_interaction.deferred {
                        application_interaction.followup().content(content).await?;
                    } else {
                        application_interaction
                            .reply(
                                InteractionResponseDataBuilder::new()
                                    .content(content)
                                    .flags(MessageFlags::EPHEMERAL)
                                    .build(),
                            )
                            .await?;
                    }
                }
                _ => (),
            }
        }
        _ => (),
    }

    Ok(())
}
