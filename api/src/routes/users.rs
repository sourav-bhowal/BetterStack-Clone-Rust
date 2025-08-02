use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use database::{db::Database, models::users::UserMethods};
use poem::{handler, http::StatusCode, web::{Data, Json}, Error};
use jsonwebtoken::{encode, Header, EncodingKey};
use crate::structs::{req_inputs::CreateUserRequest, req_outputs::{CreateUserResponse, SignInUserResponse}};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

// create a new user
#[handler]
pub fn sign_up_user(Json(data): Json<CreateUserRequest>, db: Data<&Arc<Mutex<Database>>>) 
    -> Result<Json<CreateUserResponse>, Error> {
        let email: String = data.email;
        let password: String = data.password;

        let mut database = db.lock().expect("Failed to lock database");

        match database.sign_up(email, password) {
            Ok(user_id) => {
                let response: CreateUserResponse = CreateUserResponse {
                    id: user_id.clone()
                };
                Ok(Json(response))
            }
            Err(_) => {
                Err(Error::from_string(
                    "User already exists",
                    StatusCode::CONFLICT,
                ))
            },
        }
}

// sign in an existing user
#[handler]
pub fn sign_in_user(Json(data): Json<CreateUserRequest>, db: Data<&Arc<Mutex<Database>>>) 
    -> Result<Json<SignInUserResponse>, Error> {
        let email: String = data.email;
        let password: String = data.password;

        let mut database = db.lock().expect("Failed to lock database");

        match database.sign_in(email, password) {
            Ok(user) => {
                
                let claims = Claims {
                    sub: user.id.clone(),
                    exp: 3600, // Token expiration time in seconds
                };

                let token = encode(
                    &Header::default(),
                    &claims,
                    &EncodingKey::from_secret("your_secret_key".as_ref()),
                ).expect("Failed to generate token");

                let response: SignInUserResponse = SignInUserResponse {
                    id: user.id,
                    jwt: token,
                };

                Ok(Json(response))
            }
            Err(_) => {
                Err(Error::from_string(
                    "Invalid email or password",
                    StatusCode::UNAUTHORIZED,
                ))
            },
        }
}
