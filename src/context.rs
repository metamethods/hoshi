use rand::rngs::StdRng;
use tokio::sync::Mutex;
use twilight_http::{Client as HttpClient, client::InteractionClient};
use twilight_model::oauth::Application as UserApplication;

#[derive(Debug)]
pub struct BotContext {
    pub http_client: HttpClient,
    pub user_application: UserApplication,
    pub rng: Mutex<StdRng>,
}

impl BotContext {
    pub fn interaction(&self) -> InteractionClient<'_> {
        self.http_client.interaction(self.user_application.id)
    }
}
