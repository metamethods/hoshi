use std::sync::Arc;

use rand::RngExt;
use twilight_model::application::command::{Command, CommandType};
use twilight_util::builder::command::{CommandBuilder, StringBuilder};

use crate::{
    commands::{ALL_CONTEXTS, ALL_INTEGRATIONS},
    context::BotContext,
    error::BotResult,
    interaction::ApplicationCommandInteraction,
    resolver::ApplicationCommandInteractionChatInputDataResolver,
    response::BotResponse,
};

pub fn schema() -> Command {
    CommandBuilder::new("roll", "roll from a given range", CommandType::ChatInput)
        .option(
            StringBuilder::new("range", "the range to roll from (form of min..max)")
                .required(true)
                .build(),
        )
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
    context: Arc<BotContext>,
) -> BotResult<Option<BotResponse>> {
    let range_string = interaction
        .data_resolver
        .get_string_option("range")
        .ok_or(tl!(interaction, "errors.option.missing", name = "range"))?;
    let (min, max): (i64, i64) = {
        let (left, right) = range_string
            .split_once("..")
            .ok_or(tl!(interaction, "commands.roll.errors.invalid_syntax"))?;
        (left.trim().parse()?, right.trim().parse()?)
    };

    if min > max {
        Err(tl!(interaction, "commands.roll.errors.min_gt_max"))?
    }

    let rolled = context.rng.lock().await.random_range(min..=max);

    Ok(Some(BotResponse::Content(rolled.to_string())))
}
