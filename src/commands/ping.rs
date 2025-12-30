use chrono::{DateTime, Utc};
use twilight_util::builder::InteractionResponseDataBuilder;

use crate::{BotResult, interaction::ApplicationCommandInteraction};

pub async fn command(
    command_interaction: &mut ApplicationCommandInteraction<'_, '_>,
) -> BotResult<()> {
    let now_utc = Utc::now();

    command_interaction
        .reply(
            InteractionResponseDataBuilder::new()
                .content(t_application_interaction!(
                    command_interaction.application_interaction,
                    "command.ping.initial_response"
                ))
                .build(),
        )
        .await?;

    let message = command_interaction
        .application_interaction
        .get_response()
        .await?
        .model()
        .await?;
    let message_sent_utc: DateTime<Utc> = message.timestamp.iso_8601().to_string().parse()?;

    command_interaction
        .update_response()
        .content(Some(
            t_application_interaction!(
                command_interaction.application_interaction,
                "command.ping.response",
                duration = (message_sent_utc - now_utc).num_milliseconds()
            )
            .as_ref(),
        ))
        .await?;

    Ok(())
}
