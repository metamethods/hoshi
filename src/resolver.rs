use twilight_model::{
    application::interaction::{
        InteractionChannel, InteractionDataResolved,
        application_command::{CommandData, CommandDataOption, CommandOptionValue},
    },
    channel::{Attachment, Message},
    guild::Role,
    user::User,
};

use crate::Mentionable;

pub trait InteractionDataResolver {}

#[derive(Debug)]
pub struct ApplicationCommandInteractionChatInputDataResolver {
    pub command: String,
    pub subcommand_group: Option<String>,
    pub subcommand: Option<String>,
    command_options: Vec<CommandDataOption>,
    command_resolved: Option<InteractionDataResolved>,
}

impl ApplicationCommandInteractionChatInputDataResolver {
    pub fn new(
        command: String,
        subcommand_group: Option<String>,
        subcommand: Option<String>,
        options: Vec<CommandDataOption>,
        resolved: Option<InteractionDataResolved>,
    ) -> Self {
        Self {
            command,
            subcommand_group,
            subcommand,
            command_options: options,
            command_resolved: resolved,
        }
    }

    pub fn from_commmand_data(command_data: Box<CommandData>) -> Self {
        let mut hoisted_options = command_data.options;

        let mut subcommand_group_name = None;

        if let Some(CommandDataOption {
            name,
            value: CommandOptionValue::SubCommandGroup(subcommand_group_options),
        }) = hoisted_options.first_mut()
        {
            subcommand_group_name = Some(name.clone());
            hoisted_options = std::mem::take(subcommand_group_options);
        }

        let mut subcommand_name = None;

        if let Some(CommandDataOption {
            name,
            value: CommandOptionValue::SubCommand(subcommand_options),
        }) = hoisted_options.first_mut()
        {
            subcommand_name = Some(name.clone());
            hoisted_options = std::mem::take(subcommand_options);
        }

        Self {
            command: command_data.name,
            subcommand_group: subcommand_group_name,
            subcommand: subcommand_name,
            command_options: hoisted_options,
            command_resolved: command_data.resolved,
        }
    }

    fn get_option<OptionName: AsRef<str>>(
        &self,
        option_name: OptionName,
    ) -> Option<&CommandDataOption> {
        let option_name = option_name.as_ref();

        self.command_options
            .iter()
            .find(|option| option.name == option_name)
    }

    pub fn get_string_option<OptionName: AsRef<str>>(
        &self,
        option_name: OptionName,
    ) -> Option<String> {
        let CommandOptionValue::String(ref value) = self.get_option(option_name)?.value else {
            return None;
        };

        Some(value.clone())
    }

    pub fn get_integer_option<OptionName: AsRef<str>>(
        &self,
        option_name: OptionName,
    ) -> Option<i64> {
        let CommandOptionValue::Integer(value) = self.get_option(option_name)?.value else {
            return None;
        };

        Some(value)
    }

    pub fn get_number_option<OptionName: AsRef<str>>(
        &self,
        option_name: OptionName,
    ) -> Option<f64> {
        let CommandOptionValue::Number(value) = self.get_option(option_name)?.value else {
            return None;
        };

        Some(value)
    }

    pub fn get_boolean_option<OptionName: AsRef<str>>(
        &self,
        option_name: OptionName,
    ) -> Option<bool> {
        let CommandOptionValue::Boolean(value) = self.get_option(option_name)?.value else {
            return None;
        };

        Some(value)
    }

    pub fn get_user_option<OptionName: AsRef<str>>(
        &self,
        option_name: OptionName,
    ) -> Option<&User> {
        let CommandOptionValue::User(value) = self.get_option(option_name)?.value else {
            return None;
        };

        self.command_resolved.as_ref()?.users.get(&value)
    }

    pub fn get_channel_option<OptionName: AsRef<str>>(
        &self,
        option_name: OptionName,
    ) -> Option<&InteractionChannel> {
        let CommandOptionValue::Channel(value) = self.get_option(option_name)?.value else {
            return None;
        };

        self.command_resolved.as_ref()?.channels.get(&value)
    }

    pub fn get_role_option<OptionName: AsRef<str>>(
        &self,
        option_name: OptionName,
    ) -> Option<&Role> {
        let CommandOptionValue::Role(value) = self.get_option(option_name)?.value else {
            return None;
        };

        self.command_resolved.as_ref()?.roles.get(&value)
    }

    pub fn get_mentionable_option<OptionName: AsRef<str>>(
        &self,
        option_name: OptionName,
    ) -> Option<Mentionable<'_>> {
        let CommandOptionValue::Mentionable(value) = self.get_option(option_name)?.value else {
            return None;
        };

        let resolved = self.command_resolved.as_ref()?;

        if let Some(user) = resolved.users.get(&value.cast()) {
            Some(Mentionable::User(user))
        } else if let Some(role) = resolved.roles.get(&value.cast()) {
            Some(Mentionable::Role(role))
        } else {
            None
        }
    }

    pub fn get_attachment_option<OptionName: AsRef<str>>(
        &self,
        option_name: OptionName,
    ) -> Option<&Attachment> {
        let CommandOptionValue::Attachment(value) = self.get_option(option_name)?.value else {
            return None;
        };

        self.command_resolved.as_ref()?.attachments.get(&value)
    }
}

impl InteractionDataResolver for ApplicationCommandInteractionChatInputDataResolver {}

#[derive(Debug)]
pub struct ApplicationCommandInteractionMessageDataResolver {
    pub command: String,
    pub message: Message,
}

impl ApplicationCommandInteractionMessageDataResolver {
    pub fn new(command: String, message: Message) -> Self {
        Self { command, message }
    }
}

impl InteractionDataResolver for ApplicationCommandInteractionMessageDataResolver {}

#[derive(Debug)]
pub struct ApplicationCommandInteractionUserDataResolver {
    pub command: String,
    pub user: User,
}

impl ApplicationCommandInteractionUserDataResolver {
    pub fn new(command: String, user: User) -> Self {
        Self { command, user }
    }
}

impl InteractionDataResolver for ApplicationCommandInteractionUserDataResolver {}
