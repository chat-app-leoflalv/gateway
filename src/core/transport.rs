use async_trait::async_trait;
use axum::Json;
use serde::{de::DeserializeOwned, Serialize};

pub mod nats_transport;

#[async_trait]
pub trait Transport {
    async fn connect(server: &str) -> anyhow::Result<Self>
    where
        Self: Sized;

    async fn request<P, R>(
        &self,
        pattern: &'static str,
        payload: &Json<P>,
    ) -> anyhow::Result<Json<R>>
    where
        P: Serialize + Send + Sync,
        R: DeserializeOwned + Send;

    async fn emit<P>(&self, pattern: &'static str, payload: &Json<P>) -> anyhow::Result<()>
    where
        P: Serialize + Send + Sync;
}
