use tokio_postgres::NoTls;
use tokio_postgres::Client;
use warp::{Filter, Rejection};

mod users;
use users::*;

async fn query_db(client: &Client) -> Result<String, Rejection> {
    // Add db query logic here
    // Example: let result = client.query("SELECT * FROM table", &[]).await;
    Ok("Data from the database".to_string())
}

#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=dsdb=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "dsdb=info");
    }
    pretty_env_logger::init();

    // Setup db connection
    let (client, connection) = 
        tokio_postgres::connect("host=localhost port=8080 user=username password=password dbname=dsdb", NoTls).await.unwrap();

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Define a warp filter
    //let api = warp::path("api")
        //.and(warp::path("data"))
        //.and(warp::get())
        //.and(warp::any().map(move || client.clone()))
        //.and_then(query_db);

    // GET /hello/warp => 200 OK with body "Hello, warp!"
    //let hello = warp::path!("hello" / String)
        //.map(|name| format!("Hello, {}!", name));

    // Secret key for JWT verification (to replace using env)
    let secret_key = "your_secret_key";

    let login_route = warp::path!("api" / "login")
        .and(warp::post())
        .and_then(login_handler);

    let logout_route = warp::path!("api" / "logout")
        .and(warp::post())
        .and_then(logout_handler);

    // Protect routes with authentication middleware
    let authenticated_user_route = warp::path!("api" / "user")
        .and(users::authenticate(secret_key))
        .and(warp::get().and_then(users::get_user_handler))
        .or(warp::post().and_then(users::create_user_handler))
        .or(warp::put().and_then(users::update_user_handler));

    let user_list_route = warp::path!("api" / "user" / "list")
        .and(users::authenticate(secret_key))
        .and(warp::get().and_then(users::list_users_handler));

    let routes = login_route
        .or(logout_route)
        .or(authenticated_user_route)
        .or(user_list_route);

    let db = models::blank_db();

    let api = filters::query_db(db);
    // View access logs by setting `RUST_LOG=dsdb`.
    let routes = api.with(warp::log("dsdb"));

    // Start the server
    warp::serve(routes)
        .run(([127,0,0,1], 8080))
        .await;
}
