use mongodb_logger::logger::Logger;

use mongodb::{options::ClientOptions, Client};

use log::info;

#[async_std::main]
async fn main() {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017")
        .await
        .unwrap();
    client_options.app_name = Some("mongodb_logger".to_string());
    let client = Client::with_options(client_options).unwrap();

    mongodb_logger::logger::init(client.clone());

    info!("Hi");
    println!("Hello, world!");
}
