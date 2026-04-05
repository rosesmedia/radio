use std::path::PathBuf;

use miette::{Context, IntoDiagnostic};
use tokio::{io::AsyncWriteExt as _, net::UnixStream};

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
        todo!("this allows for path traversal");
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

    async fn send_command(&mut self, command: &[u8]) -> Result<(), std::io::Error> {
        let Self(stream) = self;

        // make sure connection is writable
        stream.writable().await?;

        // send command
        stream.write(command).await?;
        stream.write(Self::ACTION_SEND_COMMAND).await?;

        // flush connection
        stream.flush().await
    }

    pub async fn set_source(&mut self, source: &Source) -> miette::Result<()> {
        let command = format!("var.set source={}", source.id());
        self.send_command(command.as_bytes())
            .await
            .into_diagnostic()
            .with_context(|| "failed to write to unix socket")
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

    pub fn from_id(id: u8) -> Option<Self> {
        match id {
            1 => Some(Source::Live),
            2 => Some(Source::PreStream),
            3 => Some(Source::PostStream),
            4 => Some(Source::TechnicalDifficulties),
            _ => None,
        }
    }
}
