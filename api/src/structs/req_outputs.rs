use serde::{Deserialize, Serialize};

// Use Macro to derive Serialize and Deserialize traits
#[derive(Deserialize, Serialize)]
pub struct CreateWebsiteResponse {
    pub id: String,
}

#[derive(Deserialize, Serialize)]
pub struct CreateUserResponse {
    pub id: String,
}

#[derive(Deserialize, Serialize)]
pub struct SignInUserResponse {
    pub id: String,
    pub jwt: String,
}

#[derive(Deserialize, Serialize)]
pub struct GetWebsiteResponse {
    pub id: String,
    pub url: String,
    pub user: User,
}

#[derive(Deserialize, Serialize)]
pub struct User {
    pub id: String,
    pub email: String,
}