use std::sync::Arc;
use mongodb::{Client,Database};
use mongodb::options::ClientOptions;

#[derive(Clone)]
pub struct State {
    pub db: Arc<Database>,
}

impl State {
    pub async fn default() -> State {
        let opt: ClientOptions = ClientOptions::parse("mongodb://127.0.0.1:27017")
        .await.expect("Error on mongo connection string");
        let db: Database = Client::with_options(opt)
            .expect("Error on connecting mongodb")
            .database("test");
        State {
            db: Arc::new(db)
        }
    }
}