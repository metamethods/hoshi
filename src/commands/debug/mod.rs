use twilight_model::application::command::Command;

pub mod error;

#[cfg(debug_assertions)]
pub fn command_schemas() -> Vec<Command> {
    vec![error::schema()]
}

#[cfg(not(debug_assertions))]
pub fn command_schemas() -> Vec<Command> {
    vec![]
}
