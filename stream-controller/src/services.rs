use miette::{Context, IntoDiagnostic};
use systemd_zbus::{ManagerProxy, Mode};

const STREAMER_UNIT: &str = "liq-streamer";

#[derive(Clone)]
pub struct ServiceManager {
    connection: zbus::Connection,
}

impl ServiceManager {
    pub async fn new() -> miette::Result<Self> {
        let connection = zbus::Connection::system()
            .await
            .into_diagnostic()
            .with_context(|| "connecting to system dbus")?;
        Ok(Self { connection })
    }

    async fn get_proxy(&self) -> miette::Result<ManagerProxy<'_>> {
        ManagerProxy::new(&self.connection)
            .await
            .into_diagnostic()
            .with_context(|| "getting manager proxy")
    }

    async fn start(&self, service: &str) -> miette::Result<()> {
        tracing::info!(service, "starting unit");
        self.get_proxy()
            .await?
            .start_unit(service, Mode::Replace)
            .await
            .into_diagnostic()
            .with_context(|| format!("restarting unit {service}"))?;
        Ok(())
    }

    async fn stop(&self, service: &str) -> miette::Result<()> {
        tracing::info!(service, "stopping unit");
        self.get_proxy()
            .await?
            .stop_unit(service, Mode::Replace)
            .await
            .into_diagnostic()
            .with_context(|| format!("stopping unit {service}"))?;
        Ok(())
    }

    fn streamer_unit_name(&self, id: &str) -> String {
        format!("{STREAMER_UNIT}@{id}.service")
    }

    pub async fn start_streamer(&self, id: &str) -> miette::Result<()> {
        self.start(&self.streamer_unit_name(id))
            .await
            .with_context(|| format!("starting streamer {id}"))
    }

    pub async fn stop_streamer(&self, id: &str) -> miette::Result<()> {
        self.stop(&self.streamer_unit_name(id))
            .await
            .with_context(|| format!("starting streamer {id}"))
    }
}
