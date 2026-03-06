use twilight_model::{
    application::{command::Command, interaction::InteractionContextType},
    oauth::ApplicationIntegrationType,
};

pub mod debug;

pub mod greet;
pub mod ping;
pub mod roll;

pub mod echo;

pub const ALL_CONTEXTS: [InteractionContextType; 3] = [
    InteractionContextType::Guild,
    InteractionContextType::BotDm,
    InteractionContextType::PrivateChannel,
];

pub const ALL_INTEGRATIONS: [ApplicationIntegrationType; 2] = [
    ApplicationIntegrationType::GuildInstall,
    ApplicationIntegrationType::UserInstall,
];

pub fn command_schemas() -> Vec<Command> {
    vec![
        ping::schema(),
        roll::schema(),
        echo::schema(),
        greet::schema(),
    ]
    .into_iter()
    .chain(debug::command_schemas())
    .collect()
}
