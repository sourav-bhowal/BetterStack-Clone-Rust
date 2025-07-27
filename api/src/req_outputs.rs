use serde::{Deserialize, Serialize};

// Use Macro to derive Serialize and Deserialize traits
#[derive(Deserialize, Serialize)]
pub struct CreateWebsiteResponse {
    pub id: String,
}