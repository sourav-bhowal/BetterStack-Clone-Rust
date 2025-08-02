use std::sync::{Arc, Mutex};
use database::{db::Database, models::websites::WebsiteMethods};
use poem::{handler, http::StatusCode, web::{Data, Json, Path}, Error};
use crate::structs::{req_inputs::CreateWebsiteRequest, req_outputs::{CreateWebsiteResponse, GetWebsiteResponse, User}};

// get website by ID
#[handler]
pub fn get_website(Path(website_id): Path<String>, db:Data<&Arc<Mutex<Database>>>) 
    -> Result<Json<GetWebsiteResponse>, Error> {

        let mut database = db.lock().expect("Failed to lock database");
        
        match database.get_website(&website_id) {
            Ok(website_with_user) => {
                let response: GetWebsiteResponse = GetWebsiteResponse {
                    id: website_with_user.website.id.clone(),
                    url: website_with_user.website.url.clone(),
                    user: User {
                        id: website_with_user.user.id.clone(),
                        email: website_with_user.user.email.clone(),
                    },
                };
                Ok(Json(response))
            }
            Err(_) => {
                Err(Error::from_string(
                    format!(
                    "Website with ID {} not found", website_id),
                    StatusCode::NOT_FOUND,
                ))
            },
        }
}

// create a new website
#[handler]
pub fn create_website(Json(data): Json<CreateWebsiteRequest>, db: Data<&Arc<Mutex<Database>>>) 
    -> Result<Json<CreateWebsiteResponse>, Error> {
        let url: String = data.url;
        let user_id: String = data.user_id;

        let mut database = db.lock().expect("Failed to lock database");

        match database.create_website(url, user_id) {
            Ok(website_id) => {
                let response: CreateWebsiteResponse = CreateWebsiteResponse {
                    id: website_id.clone(),
                };
                Ok(Json(response))
            }
            Err(_) => {
                Err(Error::from_string(
                    "Failed to create website",
                    StatusCode::INTERNAL_SERVER_ERROR,
                ))
            },
        }
}
