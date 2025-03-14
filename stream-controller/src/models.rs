use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngestPoint {
    pub id: String,
    pub name: String,
    pub icecast_server: String,
    pub icecast_mount: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngestPoints {
    pub ingest_points: Vec<IngestPoint>,
}

#[derive(Debug, Deserialize)]
pub enum StreamState {
    Pending,
    Live,
    Complete,
}
