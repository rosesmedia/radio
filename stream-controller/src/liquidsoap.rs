use std::path::PathBuf;

use tokio::{io::AsyncWriteExt as _, net::UnixStream};

#[derive(Debug, Clone)]
pub struct LiquidsoapClient {
    path: PathBuf,
}

#[derive(Debug, Clone, Copy)]
pub enum Source {
    Live,
    PreStream,
    PostStream,
    TechnicalDifficulties,
}

impl LiquidsoapClient {
    const ACTION_SEND_COMMAND: &'static [u8] = b"\n";

    fn new<P: Into<PathBuf>>(path: P) -> Self {
        Self { path: path.into() }
    }

    async fn create_connection(&self) -> Result<UnixStream, std::io::Error> {
        UnixStream::connect(&self.path).await
    }

    async fn send_command(
        &self,
        connection: &mut UnixStream,
        command: &[u8],
    ) -> Result<(), std::io::Error> {
        // make sure connection is writable
        connection.writable().await?;

        // send command
        connection.write(command).await?;
        connection.write(Self::ACTION_SEND_COMMAND).await?;

        // flush connection
        connection.flush().await
    }

    pub async fn set_source(&self, source: &Source) -> Result<(), std::io::Error> {
        let command = format!("var.set source={}", source.id());
        let mut connection = self.create_connection().await?;
        self.send_command(&mut connection, command.as_bytes()).await
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
}

pub fn get_client(id: &str) -> LiquidsoapClient {
    LiquidsoapClient::new(format!("/tmp/restreamer-control-{id}.sock"))
}
