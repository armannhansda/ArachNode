use mongodb::{Client, Collection, bson::doc, options::ClientOptions};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize, Deserialize, Debug)]
pub struct Page {
    pub url: String,
    pub title: String,
    pub description: String,
    pub content: String,
}

pub struct MongoDB {
    pub collection: Collection<Page>,
}

impl MongoDB {
    pub async fn init() -> Self {
        let mongo_uri = env::var("MONGODB_URI").unwrap_or_else(|_| {
            panic!("MONGODB_URI must be set to your MongoDB Atlas connection string")
        });
        let mongo_db_name =
            env::var("MONGODB_DB_NAME").unwrap_or_else(|_| "search_engine".to_string());

        let options = ClientOptions::parse(&mongo_uri)
            .await
            .unwrap_or_else(|error| panic!("failed to parse MONGODB_URI ({mongo_uri}): {error}"));
        let client = Client::with_options(options).unwrap();

        let db = client.database(&mongo_db_name);
        db.run_command(doc! { "ping": 1 })
            .await
            .unwrap_or_else(|error| panic!("failed to connect to MongoDB Atlas: {error}"));

        let collection = db.collection::<Page>("pages");

        MongoDB { collection }
    }

    pub async fn insert_page(&self, page: Page) -> bool {
        match self.collection.insert_one(page).await {
            Ok(_) => true,
            Err(error) => {
                eprintln!("failed to insert page into MongoDB: {error}");
                false
            }
        }
    }
}
