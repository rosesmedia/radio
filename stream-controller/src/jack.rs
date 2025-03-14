// TODO: maybe run an actual jack client rather than shelling out to the CLI tools

use miette::{Context, IntoDiagnostic};
use tokio::process::Command;

#[derive(Clone)]
pub struct JackManager {}

impl JackManager {
    pub fn new() -> Self {
        Self {}
    }

    async fn patch(&self, input: &str, output: &str) -> miette::Result<()> {
        let result = Command::new("/usr/bin/jack_connect")
            .arg(input)
            .arg(output)
            .status()
            .await
            .into_diagnostic()
            .with_context(|| "executing jack_connect")?;
        if !result.success() {
            miette::bail!("command returned non-zero exit code: {:?}", result.code());
        }
        Ok(())
    }

    async fn patch_stereo(&self, input: &str, output: &str) -> miette::Result<()> {
        self.patch(&format!("{input}:out_0"), &format!("{output}:in_0"))
            .await
            .with_context(|| format!("patching `{input}:out_0` to `{output}:in_0`"))?;
        self.patch(&format!("{input}:out_1"), &format!("{output}:in_1"))
            .await
            .with_context(|| format!("patching `{input}:out_1` to `{output}:in_1`"))?;
        Ok(())
    }

    fn ingest_jack_name(&self, ingest_id: &str) -> String {
        format!("ingest_{ingest_id}")
    }

    fn streamer_jack_name(&self, stream_id: &str) -> String {
        format!("streamer_{stream_id}")
    }

    pub async fn patch_ingest_to_stream(
        &self,
        ingest_id: &str,
        stream_id: &str,
    ) -> miette::Result<()> {
        self.patch_stereo(
            &self.ingest_jack_name(ingest_id),
            &self.streamer_jack_name(stream_id),
        )
        .await
    }
}
