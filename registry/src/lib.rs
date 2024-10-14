use std::sync::Arc;

use adapter::{
    database::ConnectionPool,
    repository::health::{self, HealthCheclRepositryImpl},
};
use kernel::repository::health::HealthCheclRepositry;

#[derive(Clone)]
pub struct AppRegistry {
    health_check_repository: Arc<dyn HealthCheclRepositry>,
}

impl AppRegistry {
    pub fn new(pool: ConnectionPool) -> Self {
        let health_check_repository = Arc::new(HealthCheclRepositryImpl::new(pool.clone()));
        Self {
            health_check_repository,
        }
    }

    pub fn health_check_repository(&self) -> Arc<dyn HealthCheclRepositry> {
        self.health_check_repository.clone()
    }
}
