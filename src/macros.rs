#[macro_export]
macro_rules! t_application_interaction {
    ($application_interaction:expr, $($all:tt)*) => {
        t!($($all)*, locale = &$application_interaction.event_interaction.locale.clone().unwrap_or("en".into()))
    };
}

#[macro_export]
macro_rules! command_option_choice_vec {
    [$($v:literal),*] => {
        vec![$(
            twilight_model::application::command::CommandOptionChoice {
                name: $v.into(),
                value: twilight_model::application::command::CommandOptionChoiceValue::String($v.into()),
                name_localizations: None
            }
        ),*]
    };
}

#[macro_export]
macro_rules! static_autocomplete_fn {
    ($name:ident, $($v:literal),*) => {
        fn $name<Partial: AsRef<str>>(partial: Partial) -> Vec<twilight_model::application::command::CommandOptionChoice> {
            command_option_choice_vec![$($v),*].into_iter().filter(|option| option.name.contains(partial.as_ref())).collect()
        }
    };

    (pub $name:ident, $($v:literal),*) => {
        pub fn $name<Partial: AsRef<str>>(partial: Partial) -> Vec<twilight_model::application::command::CommandOptionChoice> {
            command_option_choice_vec![$($v),*].into_iter().filter(|option| option.name.contains(partial.as_ref())).collect()
        }
    };
}
