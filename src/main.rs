use warp::Filter;
use tokio_postgres::{NoTls, Client};

async fn query_db(client: &Client) -> Result<String, Rejection> {
    // Add db query logic here
    // Example: let result = client.query("SELECT * FROM table", &[]).await;
    Ok("Data from the database".to_string())
}

#[tokio::main]
fn main() {
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
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    let db = models::blank_db();

    let api = filters::query_db(db);
    // View access logs by setting `RUST_LOG=dsdb`.
    let routes = api.with(warp::log("dsdb"));

    // Start the server
    warp::serve(api)
        .run(([127,0,0,1], 8080))
        .await;
}
