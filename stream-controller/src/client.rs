use reqwest::{Client, ClientBuilder};

use crate::models::IngestPoints;

// TODO: change to roses.media domain when deployed
const USER_AGENT: &str = "roses-radio-tx/1.0 (+https://ury.org.uk)";

pub struct ApiClient {
    inner: Client,
    api_base: String,
    api_token:String,
}

impl ApiClient {
    pub fn new(api_base: impl Into<String>, api_token: impl Into<String>) -> Self {
        Self {
            inner: ClientBuilder::new()
                .user_agent(USER_AGENT)
                .build()
                .expect("failed to build client"),
            api_base: api_base.into(),
            api_token: api_token.into(),
        }
    }

    pub async fn get_ingest_points(&self) -> reqwest::Result<IngestPoints> {
        self.inner
            .get(format!("{}/api/ingest", self.api_base))
            .header("Authorization", format!("Bearer {}", self.api_token))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
    }
}
