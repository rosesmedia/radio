use services::ServiceManager;

pub mod auth;
pub mod liquidsoap;
pub mod response;
pub mod routes;
pub mod services;

#[derive(Clone)]
pub struct AppState {
    pub service_manager: ServiceManager,
}
