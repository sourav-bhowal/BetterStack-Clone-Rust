// Import poem (similar to EXPRESS)
use poem::{get, listener::TcpListener, middleware::AddData, post, EndpointExt, Route, Server};
use database::{db::Database};
use std::io::Error;
use std::sync::{Arc, Mutex};
use api::routes::websites::{get_website, create_website};
use api::routes::users::{sign_up_user, sign_in_user};

/*
    NOTE:
    The "Data" type in poem is used to share state across handlers.
    It allows you to pass shared data (like a database connection) to your handlers without having to pass it explicitly in each function signature.

    "Arc" is an atomic reference counter, which allows multiple threads to own the same data. 
    In this case, it is used to share the database instance across multiple requests.
    It ensures that the database can be accessed safely from multiple threads without running into issues with ownership or borrowing.

    "Mutex" is a mutual exclusion primitive that provides thread-safe access to the data (in this case, the database).
    It ensures that only one thread can access the database at a time, preventing data races and inconsistencies.
    i.e. when one thread (eg; get_website, create_website, sign_up_user, sign_in_user) is reading or writing to the database, 
    other threads must wait until it is done. So, they cannot access the database simultaneously.
    This is important for maintaining data integrity in a multi-threaded environment. so they 'lock' the database before accessing it.
    When the thread is done, it 'unlocks' the database, allowing other threads to access it.
    This is a common pattern in web applications to ensure that the database is accessed safely and consistently
    across multiple requests.
*/

// Main function to run the server
#[tokio::main(flavor = "multi_thread")]
async fn main() -> Result<(), Error> {

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