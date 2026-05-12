use crate::structs::user::User;
use mongodb::Database;
use mongodb::bson::doc;
use mongodb::options::UpdateOptions;

const PASS: &str = "password";

pub async fn users(db: &Database) -> Result<(), Box<dyn std::error::Error>> {
    let users = vec![
        User {
            _id: "admin@email.com".to_string(),
            name: "admin".to_string(),
            pass: crate::helper::hash_password(PASS),
        },
        User {
            _id: "user@email.com".to_string(),
            name: "user".to_string(),
            pass: crate::helper::hash_password(PASS),
        },
    ];
    let collection = db.collection::<User>("users");
    for user in users {
        let filter = doc! { "_id": &user._id };
        let user_doc = mongodb::bson::to_document(&user)?;
        let update = doc! { "$set": user_doc };
        let options = UpdateOptions::builder().upsert(true).build();
        collection.update_one(filter, update, options).await?;
    }
    Ok(())
}
