use std::{env, sync::Arc, time::Duration};

use hoshi::{ASSETS_DIR, BotResult, assets, commands, context::BotContext, events};
use tokio::task::JoinSet;
use twilight_gateway::{Config, ConfigBuilder, EventTypeFlags, Intents, Shard, StreamExt};
use twilight_http::Client as HttpClient;

const INTENTS: Intents = Intents::empty();
const EVENT_FLAGS: EventTypeFlags = EventTypeFlags::from_bits(
    EventTypeFlags::READY.bits() | EventTypeFlags::INTERACTION_CREATE.bits(),
)
.unwrap();

async fn shard_handler(mut shard: Shard, context: BotContext) {
    while let Some(some_event) = shard.next_event(EVENT_FLAGS).await {
        let event = match some_event {
            Ok(event) => event,
            Err(_) => continue,
        };

        tokio::spawn({
            let context = context.clone();

            async move {
                if let Err(error) = events::handle_event(event, context).await {
                    eprintln!("{error:?}");
                }
            }
        });
    }
}

async fn reshard(http: &Arc<HttpClient>, config: Config, context: BotContext) -> BotResult<()> {
    let shards = twilight_gateway::create_recommended(
        &http,
        config, //Config::new(token.clone(), INTENTS),
        |_, builder: ConfigBuilder| builder.build(),
    )
    .await?;

    println!("spawning {} shard(s)", shards.len());

    let mut join_set = JoinSet::new();

    for shard in shards {
        join_set.spawn(tokio::spawn(shard_handler(shard, context.clone())));
    }

    join_set.join_all().await;

    Ok(())
}

#[tokio::main]
async fn main() -> BotResult<()> {
    let _ = dotenvy::dotenv();

    let token = env::var("DISCORD_TOKEN")?;

    let Ok(assets) = assets::load_assets(&ASSETS_DIR) else {
        return Err("failed to load assets".into());
    };

    let http = Arc::new(HttpClient::new(token.clone()));

    let app = http.current_user_application().await?.model().await?;
    let http_interaction = http.interaction(app.id);

    http_interaction
        .set_global_commands(&commands::commands())
        .await?;

    let context = BotContext::new(app.id, http.clone(), reqwest::Client::new(), assets);
    let config = Config::new(token.clone(), INTENTS);

    loop {
        tokio::select! {
            _ = reshard(&http, config.clone(), context.clone()) => {}
            _ = tokio::time::sleep(Duration::from_hours(24)) => {}
        }
    }
}
