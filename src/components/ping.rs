use twilight_model::channel::message::{
    Component,
    component::{Container, TextDisplay},
};

pub fn component(round_trip_time: u128, one_way_trip_time: u128, locale: String) -> Component {
    Component::Container(Container {
        id: None,
        accent_color: None,
        spoiler: None,
        components: [Component::TextDisplay(TextDisplay {
            id: None,
            content: t!(
                "components.ping.content",
                round_trip_time = round_trip_time,
                one_way_trip_time = one_way_trip_time,
                locale = locale
            )
            .into(),
        })]
        .into(),
    })
}
