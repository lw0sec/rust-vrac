use mongodb::Client;

use log::{Level, LevelFilter, Log, Metadata, Record};
use serde::{Deserialize, Serialize};

use async_std::task;

#[derive(Serialize, Deserialize)]
pub struct LoggerEntry {
    message: String,
}

pub struct Logger {
    mongodb_client: Client,
}

impl Log for Logger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        true
    }

    fn log(&self, record: &Record) {
        println!("Ok");

        let db = self.mongodb_client.database("mongodb_logger");
        let collection = db.collection::<LoggerEntry>("logs");

        let entry = LoggerEntry {
            message: "Test message".to_string(),
        };

        /*let handle = task::spawn(async move {
            collection.insert_one(&entry, None).await;
            println!("Ok");
        });*/
        std::thread::spawn(move || async move {
            collection.insert_one(&entry, None).await;
            println!("Ok2");
        });
    }

    fn flush(&self) {}
}

pub fn init(client: Client) {
    let logger = Logger {
        mongodb_client: client,
    };

    log::set_boxed_logger(Box::from(logger));
    log::set_max_level(LevelFilter::Info);
}
