use std::io::Cursor;

use image::{ImageFormat, imageops};
use twilight_model::{channel::message::MessageFlags, http::attachment::Attachment};

use crate::{
    ASSETS_DIR, BotResult, context::BotContext, download_attachment,
    interaction::ApplicationCommandInteraction,
};

pub async fn command(
    command_interaction: &mut ApplicationCommandInteraction<'_, '_>,
    context: &BotContext,
) -> BotResult<()> {
    command_interaction
        .defer_reply(MessageFlags::empty())
        .await?;

    let input_image_attachment = command_interaction.get_attachment_option("input").ok_or(
        t_application_interaction_err!(
            command_interaction.application_interaction,
            "error.command.option.required_missing",
            option_name = "input"
        ),
    )?;

    let image_buffer =
        download_attachment(&input_image_attachment, &context.reqwest_client).await?;

    let mut image = image::load_from_memory(image_buffer.as_ref())?;
    let bubble_image = image::load_from_memory(
        ASSETS_DIR
            .get_file("bubbles/tail_left.png")
            .ok_or(t_application_interaction_err!(
                command_interaction.application_interaction,
                "error.command.generic",
            ))?
            .contents(),
    )?;

    let bubble_image = bubble_image.resize_exact(
        image.width(),
        image.height() / 4,
        imageops::FilterType::Lanczos3,
    );

    imageops::overlay(&mut image, &bubble_image, 0, 0);

    let mut image_cursor = Cursor::new(vec![]);

    image.write_to(&mut image_cursor, ImageFormat::Png)?;

    command_interaction
        .update_response()
        .attachments(&[Attachment::from_bytes(
            "output.png".into(),
            image_cursor.into_inner(),
            1,
        )])
        .await?;

    Ok(())
}
