use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct EditMessageRequestDto {
    pub text: String,
}
