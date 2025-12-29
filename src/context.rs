use std::ops::Deref;
use std::sync::Arc;

use reqwest::Client as ReqwestClient;
use twilight_http::Client as HttpClient;
use twilight_http::client::InteractionClient;
use twilight_model::id::Id;
use twilight_model::id::marker::ApplicationMarker;

use crate::assets::Assets;

pub struct BotContextRef {
    pub application_id: Id<ApplicationMarker>,
    pub http: Arc<HttpClient>,
    pub reqwest_client: ReqwestClient,
    pub assets: Assets,
}

impl BotContextRef {
    pub fn new(
        application_id: Id<ApplicationMarker>,
        http: Arc<HttpClient>,
        reqwest_client: ReqwestClient,
        assets: Assets,
    ) -> Self {
        Self {
            application_id,
            http,
            reqwest_client,
            assets,
        }
    }

    pub fn interaction(&self) -> InteractionClient<'_> {
        self.http.interaction(self.application_id)
    }
}

#[derive(Clone)]
pub struct BotContext(pub Arc<BotContextRef>);

impl BotContext {
    pub fn new(
        application_id: Id<ApplicationMarker>,
        http: Arc<HttpClient>,
        reqwest_client: ReqwestClient,
        assets: Assets,
    ) -> Self {
        Self(Arc::new(BotContextRef::new(
            application_id,
            http,
            reqwest_client,
            assets,
        )))
    }
}

impl Deref for BotContext {
    type Target = BotContextRef;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
