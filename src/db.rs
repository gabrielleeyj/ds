use tokio_postgres::NoTls;
use tokio_postgres::Client;

pub trait Database {
    async fn New() -> Client;
}

async fn New() {
    // Setup db connection
    let (client, connection) = 
        tokio_postgres::connect("host=localhost port=8080 user=username password=password dbname=dsdb", notls).await.unwrap();

    tokio::spawn(async move {
        if let err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    return client;
}

async fn Insert(client: &Client, table: string, column: string, value: string) {
     // Now we can execute a simple statement that just returns its parameter.
    let rows = client
        .query("INSERT INTO $1 ($2) VALUES $3", table, column, value)
        .await?;
    println!("Value inserted: $1 to $2 ($3)", value, table, column);
}

async fn Update(client: &Client, table: string, column: string, value: string) {
    let rows = client
        .query("UPDATE $1 SET $2 = $3", table, column, value)
        .await?;

    println!("Value updated: $1 to $2 ($3)", value, table, column);
}
