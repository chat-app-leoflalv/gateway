use async_nats::Client;
use async_trait::async_trait;
use axum::{body::Bytes, Json};
use serde::{de::DeserializeOwned, Serialize};

use super::Transport;

pub struct NatsTransport {
    client: Client,
}

#[async_trait]
impl Transport for NatsTransport {
    async fn connect(server: &str) -> anyhow::Result<Self, anyhow::Error> {
        let client = async_nats::connect(server).await?;

        Ok(NatsTransport { client })
    }

    async fn request<P, R>(
        &self,
        pattern: &'static str,
        payload: &Json<P>,
    ) -> anyhow::Result<Json<R>>
    where
        P: Serialize + Send + Sync,
        R: DeserializeOwned + Send,
    {
        let serialized_payload = serde_json::to_vec(&payload.0)?;
        let response_bytes = self
            .client
            .request(pattern, Bytes::from(serialized_payload))
            .await?;

        let response_value = serde_json::from_slice::<R>(&response_bytes.payload)?;

        Ok(Json(response_value))
    }
}
