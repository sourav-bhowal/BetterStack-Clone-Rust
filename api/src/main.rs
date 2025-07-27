// Import poem (similar to EXPRESS)
use poem::{get, handler, listener::TcpListener, post, web::{Json, Path}, Route, Server};
use crate::{req_inputs::CreateWebsiteRequest, req_outputs::CreateWebsiteResponse};

// Import the necessary modules
pub mod req_inputs;
pub mod req_outputs;

// get website by ID
#[handler]
fn get_website(Path(website_id): Path<String>) -> String {
    format!("website: {}", website_id)
}

// create a new website
#[handler]
fn create_website(Json(data): Json<CreateWebsiteRequest>) -> Json<CreateWebsiteResponse> {
    let url: String = data.url;

    let response: CreateWebsiteResponse = CreateWebsiteResponse {
        id: format!("website-{}", url), // Simulating an ID generation
    };

    Json(response)
}

// main function to run the server
#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Define the routes and start the server
    let app = Route::new()
        .at("/status/:website_id", get(get_website))
        .at("/website", post(create_website));
    // Bind the server to a TCP listener
    Server::new(TcpListener::bind("0.0.0.0:9000"))
        .run(app)
        .await
}