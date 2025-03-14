use miette::{Context, IntoDiagnostic};
use systemd_zbus::{ManagerProxy, Mode};

const INGEST_UNIT: &str = "liq-ingest";

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

    async fn get_proxy(&self) -> miette::Result<ManagerProxy> {
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

    async fn restart(&self, service: &str) -> miette::Result<()> {
        tracing::info!(service, "restarting unit");
        self.get_proxy()
            .await?
            .restart_unit(service, Mode::Replace)
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

    fn ingest_unit_name(&self, id: &str) -> String {
        format!("{INGEST_UNIT}@{id}.service")
    }

    pub async fn start_ingest(&self, id: &str) -> miette::Result<()> {
        self.start(&self.ingest_unit_name(id))
            .await
            .with_context(|| format!("starting ingest {id}"))
    }

    pub async fn restart_ingest(&self, id: &str) -> miette::Result<()> {
        self.restart(&self.ingest_unit_name(id))
            .await
            .with_context(|| format!("restarting ingest {id}"))
    }

    pub async fn stop_ingest(&self, id: &str) -> miette::Result<()> {
        self.stop(&self.ingest_unit_name(id))
            .await
            .with_context(|| format!("stopping ingest {id}"))
    }
}
