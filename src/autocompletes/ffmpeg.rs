use twilight_model::application::command::CommandOptionChoice;

use crate::{BotResult, autocompletes::autocomplete_file_type};

pub async fn autocomplete(option: String, partial: String) -> BotResult<Vec<CommandOptionChoice>> {
    match option.as_str() {
        "type" => Ok(autocomplete_file_type(partial)),
        _ => Ok(vec![]),
    }
}
