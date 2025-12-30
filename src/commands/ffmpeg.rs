use tokio::process::Command;
use twilight_model::{channel::message::MessageFlags, http::attachment::Attachment};

use crate::{
    BotResult, context::BotContext, get_output_as_buffer,
    interaction::ApplicationCommandInteraction, session::FileSession,
};

const FFMPEG_ARGS_PRESETS: [(&str, &str); 1] = [(
    "gif",
    "-filter_complex \"[0:v]split[x][p];[p]palettegen[p];[x][p]paletteuse\"",
)];

pub async fn command(
    command_interaction: &mut ApplicationCommandInteraction<'_, '_>,
    context: &BotContext,
) -> BotResult<()> {
    command_interaction
        .defer_reply(MessageFlags::empty())
        .await?;

    let input_file_attachment =
        command_interaction
            .get_attachment_option("input")
            .ok_or(t_application_interaction!(
                command_interaction.application_interaction,
                "error.command.option.required_missing",
                option_name = "input"
            ))?;
    let new_file_type =
        command_interaction
            .get_string_option("type")
            .ok_or(t_application_interaction!(
                command_interaction.application_interaction,
                "error.command.option.required_missing",
                option_name = "type"
            ))?;
    let ffmpeg_arguments = command_interaction
        .get_string_option("args")
        .map(|args| args.clone());
    let use_preset = command_interaction
        .get_boolean_option("preset")
        .unwrap_or(false);
    let ffmpeg_output = command_interaction
        .get_boolean_option("output")
        .unwrap_or(false);

    let file_session = FileSession::new()?;

    file_session
        .add_file_from_attachment(&input_file_attachment, &context.reqwest_client)
        .await?;

    let input_filename = &input_file_attachment.filename;
    let output_filename = &format!("output.{new_file_type}");

    let input_path = file_session.path().join(input_filename);
    let output_path = file_session.path().join(output_filename);

    let mut command = Command::new("ffmpeg");

    command.arg("-i").arg(input_path);

    if let Some(ffmpeg_arguments) =
        ffmpeg_arguments.or(FFMPEG_ARGS_PRESETS
            .into_iter()
            .find_map(|(file_type, args)| {
                if use_preset && file_type == new_file_type {
                    Some(args.to_string())
                } else {
                    None
                }
            }))
    {
        command.args(shell_words::split(ffmpeg_arguments.as_str())?);
    }

    command.arg(output_path);

    let command_output = command.output().await?;

    let mut attachments: Vec<Attachment> = vec![];

    if let Ok(output_file_buffer) = file_session.read_file(output_filename).await {
        attachments.push(Attachment::from_bytes(
            output_filename.clone(),
            output_file_buffer,
            1,
        ));
    }

    if ffmpeg_output || attachments.is_empty() {
        attachments.push(Attachment::from_bytes(
            "ffmpeg_output.txt".into(),
            get_output_as_buffer(command_output),
            2,
        ));
    }

    command_interaction
        .update_response()
        .attachments(&attachments)
        .await?;

    Ok(())
}
