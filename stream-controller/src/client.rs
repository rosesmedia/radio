use reqwest::{Client, ClientBuilder};

use crate::models::IngestPoints;

// TODO: change to roses.media domain when deployed
const USER_AGENT: &str = "roses-radio-tx/1.0 (+https://ury.org.uk)";

pub struct ApiClient {
    inner: Client,
    api_base: String,
}

impl ApiClient {
    pub fn new(api_base: &str) -> Self {
        Self {
            inner: ClientBuilder::new()
                .user_agent(USER_AGENT)
                .build()
                .expect("failed to build client"),
            api_base: api_base.to_string(),
        }
    }

    pub async fn get_ingest_points(&self) -> reqwest::Result<IngestPoints> {
        self.inner
            .get(format!("{}/api/ingest", self.api_base))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await
    }
}
