use std::path::PathBuf;

use miette::{Context, IntoDiagnostic};
use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt as _},
    net::UnixStream,
};

#[derive(Debug, Clone)]
pub struct LiquidsoapClient {
    path: PathBuf,
}

#[derive(Debug)]
pub struct LiquidsoapConnection(UnixStream);

#[derive(Debug, Clone, Copy)]
pub enum Source {
    Live,
    PreStream,
    PostStream,
    TechnicalDifficulties,
}

impl LiquidsoapClient {
    pub fn new(stream: &str) -> Self {
        Self {
            path: format!("/tmp/restreamer-control-{stream}.sock").into(),
        }
    }

    pub async fn create_connection(&self) -> miette::Result<LiquidsoapConnection> {
        Ok(LiquidsoapConnection(
            UnixStream::connect(&self.path)
                .await
                .into_diagnostic()
                .with_context(|| "failed to connect to unix socket")?,
        ))
    }
}

impl LiquidsoapConnection {
    const ACTION_SEND_COMMAND: &'static [u8] = b"\n";

    async fn send_and_forget_command(
        &mut self,
        command: impl AsRef<[u8]>,
    ) -> Result<(), std::io::Error> {
        let Self(stream) = self;
        stream.write_all(command.as_ref()).await?;
        stream.write_all(Self::ACTION_SEND_COMMAND).await
    }

    async fn send_command(&mut self, command: impl AsRef<[u8]>) -> Result<String, std::io::Error> {
        // send command
        self.send_and_forget_command(command).await?;

        // read response
        let Self(stream) = self;

        let mut buffer = [0u8; 1024];
        let mut response = Vec::new();
        let delimiter = b"END";

        loop {
            let bytes_read = stream.read(&mut buffer).await?;
            if bytes_read == 0 {
                break;
            }
            response.extend_from_slice(&buffer[..bytes_read]);
            if let Some(position) = response
                .windows(delimiter.len())
                .position(|window| window == delimiter)
            {
                response.truncate(position);
                break;
            }
        }

        Ok(String::from_utf8_lossy(&response).trim().to_string())
    }

    pub async fn set_source(&mut self, source: &Source) -> miette::Result<()> {
        tracing::info!(?source, "setting source");

        let command = format!("var.set source={}", source.id());
        self.send_and_forget_command(command)
            .await
            .into_diagnostic()
            .with_context(|| "failed to write to unix socket")
    }

    pub async fn get_source(&mut self) -> miette::Result<Source> {
        tracing::info!("retrieving source");

        let command = "var.get source";
        let source = self
            .send_command(command)
            .await
            .into_diagnostic()
            .with_context(|| "failed to communicate with unix socket")?;

        let source_id: u8 = source
            .parse()
            .into_diagnostic()
            .with_context(|| format!("failed to parse source id: '{}'", source))?;

        Source::from_id(source_id)
            .ok_or_else(|| miette::Report::msg("invalid source received from liquidsoap"))
    }
}

impl Source {
    fn id(&self) -> u8 {
        match self {
            Source::Live => 1,
            Source::PreStream => 2,
            Source::PostStream => 3,
            Source::TechnicalDifficulties => 4,
        }
    }

    fn from_id(id: u8) -> Option<Self> {
        match id {
            1 => Some(Source::Live),
            2 => Some(Source::PreStream),
            3 => Some(Source::PostStream),
            4 => Some(Source::TechnicalDifficulties),
            _ => None,
        }
    }

    pub fn identifier(&self) -> String {
        match self {
            Source::Live => "live",
            Source::PreStream => "pre_stream",
            Source::PostStream => "post_stream",
            Source::TechnicalDifficulties => "technical_difficulties",
        }
        .to_string()
    }

    pub fn from_identifier(id: &str) -> Option<Self> {
        match id {
            "live" => Some(Source::Live),
            "pre_stream" => Some(Source::PreStream),
            "post_stream" => Some(Source::PostStream),
            "technical_difficulties" => Some(Source::TechnicalDifficulties),
            _ => None,
        }
    }

    pub fn sources() -> Vec<Source> {
        vec![
            Source::Live,
            Source::PreStream,
            Source::PostStream,
            Source::TechnicalDifficulties,
        ]
    }
}

impl<'de> Deserialize<'de> for Source {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let identifier = String::deserialize(deserializer)?;
        Self::from_identifier(&identifier)
            .ok_or_else(|| serde::de::Error::custom("invalid source provided"))
    }
}

impl Serialize for Source {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.identifier().serialize(serializer)
    }
}
