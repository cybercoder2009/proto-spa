pub mod user;

use mongodb::bson::doc;
use mongodb::Client;
use crate::db::user::User;
use crate::utilities;

pub const COL_USERS: &str = "users";

#[derive(Clone)]
pub struct Db {
    pub client: Client,
    pub db: String,
}

impl Db {
    pub async fn connect(uri: &str, db: &str) -> mongodb::error::Result<Self> {
        let client = Client::with_uri_str(uri).await?;
        Ok(Self {
            client,
            db: db.to_string(),
        })
    }

    pub async fn seeding(&self) -> Result<(), Box<dyn std::error::Error>> {
        let collection = self.client.database(&self.db).collection::<User>(COL_USERS);

        let exists = collection
            .find_one(doc! { "username": "user0" }, None)
            .await?
            .is_some();

        if !exists {
            let password_hash = bcrypt::hash("user0", bcrypt::DEFAULT_COST)?;
            let default_user = User {
                _id: utilities::_id(),
                username: "user0".to_string(),
                password: password_hash,
                t_create: utilities::now_sec(),
            };

            collection.insert_one(default_user, None).await?;
            log::info!("[db] Seeded default user 'user0'");
        }

        Ok(())
    }
}
