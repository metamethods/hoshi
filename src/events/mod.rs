use std::sync::Arc;

use twilight_gateway::Event;

use crate::{context::BotContext, error::BotResult};

pub mod interaction;
pub mod ready;

pub async fn handle_event(event: Event, context: Arc<BotContext>) -> BotResult<()> {
    match event {
        Event::Ready(ready_data) => ready::event(ready_data),
        Event::InteractionCreate(interaction_create) => {
            interaction::event(interaction_create.0, context).await?
        }
        _ => (),
    }

    Ok(())
}
