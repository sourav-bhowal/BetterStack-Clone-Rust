use serde::{Deserialize, Serialize};

// Use Macro to derive Serialize and Deserialize traits
#[derive(Deserialize, Serialize)]
pub struct CreateWebsiteRequest {
    pub url: String,
    pub user_id: String,
}

#[derive(Deserialize, Serialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
}
