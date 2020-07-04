use mongodb;
use mongodb::{Client, options::ClientOptions};
use std;

pub async fn establish_connection() -> std::result::Result<mongodb::Client, ()> {
    let database_url = std::env::var("CHATTYY_DB_CONNECTION_STRING").unwrap();
    let mut options = ClientOptions::parse(&database_url).await.unwrap();
    options.direct_connection = Some(true);
    let client = Client::with_options(options).unwrap();
    Ok(client)
}
