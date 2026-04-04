use std::path::PathBuf;

use tokio::{io::AsyncWriteExt as _, net::UnixStream};

#[derive(Debug, Clone)]
pub struct LiquidsoapClient {
    path: PathBuf,
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
}

pub fn get_client(id: &str) -> LiquidsoapClient {
    LiquidsoapClient::new(format!("/tmp/restreamer-control-{id}.sock"))
}
