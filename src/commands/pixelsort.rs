use std::io::Cursor;

use image::ImageFormat;
use imgfx::Direction;
use twilight_model::{channel::message::MessageFlags, http::attachment::Attachment};

use crate::{
    BotResult, context::BotContext, download_attachment, interaction::ApplicationCommandInteraction,
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

    let image_attachment_buffer =
        download_attachment(&input_image_attachment, &context.reqwest_client).await?;

    let image_buffer = image::load_from_memory(image_attachment_buffer.as_ref())?.to_rgba8();

    let sorted_image_buffer = imgfx::sort(
        image_buffer,
        Direction::Horizontal,
        imgfx::SortBy::Hue,
        0.,
        255.,
        false,
    );

    let mut image_cursor = Cursor::new(vec![]);

    sorted_image_buffer.write_to(&mut image_cursor, ImageFormat::Png)?;

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
