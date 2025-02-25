use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct SendMessageRequestDto {
    pub text: String,
}
