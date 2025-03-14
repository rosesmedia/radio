use services::ServiceManager;

pub mod client;
pub mod models;
pub mod routes;
pub mod services;

#[derive(Clone)]
pub struct AppState {
    pub service_manager: ServiceManager,
}
