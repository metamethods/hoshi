use twilight_gateway::Event;

use crate::{BotResult, context::BotContext};

mod interaction;
mod ready;

pub async fn handle_event(event: Event, context: BotContext) -> BotResult<()> {
    match event {
        Event::Ready(ready_data) => ready::event(ready_data),
        Event::InteractionCreate(interaction_create) => {
            interaction::event(interaction_create, context).await?
        }
        _ => (),
    }

    Ok(())
}
