use async_trait::async_trait;

#[async_trait]
pub trait HealthCheclRepositry: Send + Sync {
    async fn check_db(&self) -> bool;
}
