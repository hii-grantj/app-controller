mod error;
mod kubernetes;
#[cfg(test)]
mod mock;

pub use error::BackendError;
pub use kubernetes::KubernetesBackend;
#[cfg(test)]
pub use mock::MockBackend;

use std::future::Future;

use crate::types::{App, AppConfig, AppId, AppStatus};

/// Backend trait for the app controller.
pub trait AppControllerBackend: Send + Sync {
    /// Create a new app with the given configuration.
    fn create_app(
        &self,
        config: AppConfig,
    ) -> impl Future<Output = Result<App, BackendError>> + Send;

    /// Start the app with the given ID.
    fn start_app(&self, id: AppId) -> impl Future<Output = Result<AppStatus, BackendError>> + Send;

    /// Stop the app with the given ID.
    fn stop_app(&self, id: AppId) -> impl Future<Output = Result<AppStatus, BackendError>> + Send;

    /// Delete the app with the given ID.
    fn delete_app(&self, id: AppId) -> impl Future<Output = Result<(), BackendError>> + Send;

    /// Get the app with the given ID.
    fn get_app(&self, id: AppId) -> impl Future<Output = Result<App, BackendError>> + Send;

    /// Get all apps currently managed by the controller.
    fn get_all_apps(&self) -> impl Future<Output = Result<Vec<App>, BackendError>> + Send;
}
