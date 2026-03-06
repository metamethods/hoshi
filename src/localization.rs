use std::collections::HashMap;

use twilight_model::application::command::{Command, CommandOption};

pub fn available_localizations_of<Key: AsRef<str>>(key: Key) -> HashMap<String, String> {
    available_locales!()
        .iter()
        .map(|locale| (locale.to_string(), t!(key.as_ref(), locale = locale).into()))
        .collect()
}

pub fn localize_command_option(
    base_option_localization_key: String,
    command_option: &mut CommandOption,
) {
    let available_name_localizations =
        available_localizations_of(format!("{base_option_localization_key}.name"));

    if let Some(name_localizations) = command_option.name_localizations.as_mut() {
        name_localizations.extend(available_name_localizations);
    } else {
        command_option.name_localizations = Some(available_name_localizations);
    }

    let available_description_localizations =
        available_localizations_of(format!("{base_option_localization_key}.description"));

    if let Some(description_localizations) = command_option.description_localizations.as_mut() {
        description_localizations.extend(available_description_localizations);
    } else {
        command_option.description_localizations = Some(available_description_localizations);
    }
}

pub fn localize_command_schemas(command_schemas: &mut Vec<Command>) {
    command_schemas.iter_mut().for_each(|command| {
        let available_name_localizations =
            available_localizations_of(format!("commands.{}.name", command.name));

        if let Some(name_localizations) = command.name_localizations.as_mut() {
            name_localizations.extend(available_name_localizations);
        } else {
            command.name_localizations = Some(available_name_localizations);
        }

        if !command.description.is_empty() {
            let available_description_localizations =
                available_localizations_of(format!("commands.{}.description", command.name));

            if let Some(description_localizations) = command.description_localizations.as_mut() {
                description_localizations.extend(available_description_localizations);
            } else {
                command.description_localizations = Some(available_description_localizations);
            }
        }

        command.options.iter_mut().for_each(|command_option| {
            localize_command_option(
                format!("commands.{}.options.{}", command.name, command_option.name),
                command_option,
            )
        });
    });
}
