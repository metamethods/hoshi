use twilight_model::gateway::payload::incoming::Ready as ReadyEvent;

pub fn event(ready_event: ReadyEvent) {
    println!(
        "ready event received for {}#{}",
        ready_event.user.name,
        ready_event.user.discriminator()
    );
}
