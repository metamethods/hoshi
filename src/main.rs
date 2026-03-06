use std::sync::Arc;

use hoshi::{
    commands, context::BotContext, error::BotResult, events, localization::localize_command_schemas,
};
use rand::{SeedableRng, rngs::StdRng};
use serde::Deserialize;
use tokio::{sync::Mutex, task::JoinSet};
use twilight_gateway::{ConfigBuilder, EventTypeFlags, Intents, Shard, StreamExt};
use twilight_http::Client as HttpClient;

const INTENTS: Intents = Intents::empty();
const EVENT_FLAGS: EventTypeFlags = EventTypeFlags::from_bits(
    EventTypeFlags::READY.bits() | EventTypeFlags::INTERACTION_CREATE.bits(),
)
.unwrap();

async fn shard_handler(mut shard: Shard, context: Arc<BotContext>) {
    while let Some(some_event) = shard.next_event(EVENT_FLAGS).await {
        let event = match some_event {
            Ok(event) => event,
            Err(_) => continue,
        };

        let context = context.clone();

        tokio::spawn(async move {
            if let Err(err) = events::handle_event(event, context).await {
                eprintln!("{err:?}");
            }
        });
    }
}

async fn initialize_shards(shards: Vec<Shard>, context: Arc<BotContext>) {
    let mut join_set = JoinSet::new();

    for shard in shards {
        join_set.spawn(shard_handler(shard, context.clone()));
    }

    join_set.join_all().await;
}

#[derive(Deserialize, Debug)]
struct EnvConfig {
    token: String,
}

#[tokio::main]
async fn main() -> BotResult<()> {
    let _ = dotenvy::dotenv();
    let env_config = envy::from_env::<EnvConfig>()?;

    let http_client = HttpClient::new(env_config.token.clone());

    let user_application = http_client
        .current_user_application()
        .await?
        .model()
        .await?
        .clone();

    let context = Arc::new(BotContext {
        http_client,
        user_application,
        rng: Mutex::new(StdRng::from_rng(&mut rand::rng())),
    });

    let mut command_schemas = commands::command_schemas();

    localize_command_schemas(&mut command_schemas);

    context
        .interaction()
        .set_global_commands(&command_schemas)
        .await?;

    let gateway_config = ConfigBuilder::new(env_config.token.clone(), INTENTS).build();
    let shards = twilight_gateway::create_recommended(
        &context.http_client,
        gateway_config,
        |_, builder: ConfigBuilder| builder.build(),
    )
    .await?;

    println!("spawning {} shards", shards.len());

    initialize_shards(shards.collect(), context).await;

    Ok(())
}
