use jack::JackManager;
use services::ServiceManager;

pub mod client;
pub mod jack;
pub mod models;
pub mod routes;
pub mod services;

#[derive(Clone)]
pub struct AppState {
    pub service_manager: ServiceManager,
    pub jack_manager: JackManager,
}
