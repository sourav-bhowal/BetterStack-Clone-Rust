use std::sync::{Arc, Mutex};

// Import poem (similar to EXPRESS)
use poem::{get, handler, listener::TcpListener, middleware::AddData, post, web::{Data, Json, Path}, EndpointExt, Route, Server};
use crate::{req_inputs::{CreateUserRequest, CreateWebsiteRequest}, 
req_outputs::{CreateUserResponse, CreateWebsiteResponse, GetWebsiteResponse, SignInUserResponse}};
use database::{db::Database, models::{users::UserMethods, websites::WebsiteMethods}};

// Import the necessary modules
pub mod req_inputs;
pub mod req_outputs;

// get website by ID
#[handler]
fn get_website(Path(website_id): Path<String>, db:Data<&Arc<Mutex<Database>>>) -> Json<GetWebsiteResponse> {

    let mut database = db.lock().expect("Failed to lock database");
    
    let website = database.get_website(&website_id)
        .expect("Failed to retrieve website");

    let response: GetWebsiteResponse = GetWebsiteResponse {
        id: website.id.clone(),
        url: website.url.clone(),
    };
    Json(response)
}

// create a new website
#[handler]
fn create_website(Json(data): Json<CreateWebsiteRequest>, db: Data<&Arc<Mutex<Database>>>) -> Json<CreateWebsiteResponse> {
    let url: String = data.url;
    let user_id: String = data.user_id;

    let mut database = db.lock().expect("Failed to lock database");

    let website_id = database.create_website(url, user_id)
        .expect("Failed to create website");

    let response: CreateWebsiteResponse = CreateWebsiteResponse {
        id: website_id.clone(),
    };

    Json(response)
}

// create a new user
#[handler]
fn sign_up_user(Json(data): Json<CreateUserRequest>, db: Data<&Arc<Mutex<Database>>>) -> Json<CreateUserResponse> {
    let email: String = data.email;
    let password: String = data.password;

    let mut database = db.lock().expect("Failed to lock database");

    let user_id = database.sign_up(email, password)
        .expect("Failed to create user");

    let response: CreateUserResponse = CreateUserResponse {
        id: user_id.clone(),
    };

    Json(response)
}

#[handler]
fn sign_in_user(Json(data): Json<CreateUserRequest>, db: Data<&Arc<Mutex<Database>>>) -> Json<SignInUserResponse> {
    let email: String = data.email;
    let password: String = data.password;

    let mut database = db.lock().expect("Failed to lock database");

    let user = database.sign_in(email, password)
        .expect("Failed to sign in user");

    let response: SignInUserResponse = SignInUserResponse {
        id: user.id.clone(),
        jwt: "dummy_jwt_token".to_string(), // Replace with actual JWT generation logic
    };

    Json(response)
}

// main function to run the server
#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), std::io::Error> {

    // Initialize the database connection
    let db = Arc::new(Mutex::new(Database::new().expect("Failed to connect to the database")));

    // Define the routes and start the server
    let app = Route::new()
        .at("/status/:website_id", get(get_website))
        .at("/website", post(create_website))
        .at("/signup", post(sign_up_user))
        .at("/signin", post(sign_in_user))
        .with(AddData::new(db));

    // Bind the server to a TCP listener
    Server::new(TcpListener::bind("0.0.0.0:9000"))
        .run(app)
        .await
}