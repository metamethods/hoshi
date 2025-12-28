use twilight_model::application::command::{Command, CommandType};
use twilight_util::builder::command::{
    AttachmentBuilder, BooleanBuilder, CommandBuilder, StringBuilder,
};

use crate::{ALL_CONTEXTS, ALL_INTEGRATIONS, get_localizations_of};

pub mod ffmpeg;
pub mod ping;
pub mod pixelsort;
pub mod speechbubble;

pub fn commands() -> Vec<Command> {
    vec![
        CommandBuilder::new("ping", "pong", CommandType::ChatInput)
            .name_localizations(get_localizations_of("command.ping.name"))
            .description_localizations(get_localizations_of("command.ping.description"))
            .integration_types(ALL_INTEGRATIONS)
            .contexts(ALL_CONTEXTS)
            .build(),
        CommandBuilder::new(
            "ffmpeg",
            "runs a ffmpeg command onto a given input file",
            CommandType::ChatInput,
        )
        .name_localizations(get_localizations_of("command.ffmpeg.name"))
        .description_localizations(get_localizations_of("command.ffmpeg.description"))
        .option(
            AttachmentBuilder::new("input", "file to be used")
                .name_localizations(get_localizations_of("command.ffmpeg.options.input.name"))
                .description_localizations(get_localizations_of(
                    "command.ffmpeg.options.input.description",
                ))
                .required(true)
                .build(),
        )
        .option(
            StringBuilder::new("type", "the resulting file type")
                .name_localizations(get_localizations_of("command.ffmpeg.options.type.name"))
                .description_localizations(get_localizations_of(
                    "command.ffmpeg.options.type.description",
                ))
                .autocomplete(true)
                .required(true)
                .build(),
        )
        .option(
            StringBuilder::new("args", "extra arguments to be fed into the ffmpeg command")
                .name_localizations(get_localizations_of("command.ffmpeg.options.args.name"))
                .description_localizations(get_localizations_of(
                    "command.ffmpeg.options.args.description",
                ))
                .required(false)
                .build(),
        )
        .option(
            BooleanBuilder::new("preset", "use a preset ffmpeg arguments")
                .name_localizations(get_localizations_of("command.ffmpeg.options.preset.name"))
                .description_localizations(get_localizations_of(
                    "command.ffmpeg.options.preset.description",
                ))
                .required(false)
                .build(),
        )
        .option(
            BooleanBuilder::new("output", "show ffmpeg command output")
                .name_localizations(get_localizations_of("command.ffmpeg.options.output.name"))
                .description_localizations(get_localizations_of(
                    "command.ffmpeg.options.output.description",
                ))
                .required(false)
                .build(),
        )
        .integration_types(ALL_INTEGRATIONS)
        .contexts(ALL_CONTEXTS)
        .build(),
        CommandBuilder::new(
            "speechbubble",
            "add a speechbubble to an image",
            CommandType::ChatInput,
        )
        .name_localizations(get_localizations_of("command.speechbubble.name"))
        .description_localizations(get_localizations_of("command.speechbubble.description"))
        .option(
            AttachmentBuilder::new("input", "file to be used")
                .name_localizations(get_localizations_of(
                    "command.speechbubble.options.input.name",
                ))
                .description_localizations(get_localizations_of(
                    "command.speechbubble.options.input.description",
                ))
                .required(true)
                .build(),
        )
        .integration_types(ALL_INTEGRATIONS)
        .contexts(ALL_CONTEXTS)
        .build(),
        CommandBuilder::new(
            "pixelsort",
            "use ASDF pixel sort algorithm",
            CommandType::ChatInput,
        )
        .name_localizations(get_localizations_of("command.pixelsort.name"))
        .description_localizations(get_localizations_of("command.pixelsort.description"))
        .option(
            AttachmentBuilder::new("input", "file to be used")
                .name_localizations(get_localizations_of("command.pixelsort.options.input.name"))
                .description_localizations(get_localizations_of(
                    "command.pixelsort.options.input.description",
                ))
                .required(true)
                .build(),
        )
        .integration_types(ALL_INTEGRATIONS)
        .contexts(ALL_CONTEXTS)
        .build(),
    ]
}
