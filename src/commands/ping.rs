use twilight_util::builder::InteractionResponseDataBuilder;

use crate::{BotResult, interaction::ApplicationCommandInteraction};

pub async fn command(
    command_interaction: &mut ApplicationCommandInteraction<'_, '_>,
) -> BotResult<()> {
    command_interaction
        .reply(
            InteractionResponseDataBuilder::new()
                .content(t_application_interaction!(
                    command_interaction.application_interaction,
                    "command.ping.response"
                ))
                .build(),
        )
        .await?;

    Ok(())
}
