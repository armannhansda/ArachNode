use mongodb::{Client, Collection, bson::doc, options::ClientOptions};
use serde::{Deserialize, Serialize};

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
        let options = ClientOptions::parse("mongodb://localhost:27017")
            .await
            .unwrap();
        let client = Client::with_options(options).unwrap();

        let db = client.database("search_engine");
        let collection = db.collection::<Page>("pages");

        MongoDB { collection }
    }

    pub async fn insert_page(&self, page: Page) {
        let _ = self.collection.insert_one(page).await;
    }
}
