use twilight_model::gateway::payload::incoming::Ready;

pub fn event(ready_data: Ready) {
    println!(
        "ready event received for {}#{}",
        ready_data.user.name,
        ready_data.user.discriminator()
    );
}
