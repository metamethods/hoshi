use twilight_model::{
    channel::message::{
        Component,
        component::{Container, TextDisplay},
    },
    id::{Id, marker::InteractionMarker},
};

pub fn component(
    error_string: String,
    interaction_id: Id<InteractionMarker>,
    locale: String,
) -> Component {
    Component::Container(Container {
        id: None,
        accent_color: None,
        spoiler: None,
        components: [Component::TextDisplay(TextDisplay {
            id: None,
            content: t!(
                "components.error.content",
                error = error_string,
                interaction_id = interaction_id,
                locale = locale
            )
            .into(),
        })]
        .into(),
    })
}
