use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct GetMessageResponsePayload {
    pub data: String,
}
