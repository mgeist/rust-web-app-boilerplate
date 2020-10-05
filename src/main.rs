use async_session::{SessionStore};
use sqlx::prelude::*;
use sqlx::sqlite::SqlitePool;

mod models;

use models::User;

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    // DB Stuff
    // TODO: Remove this workaround in sqlx > 3
    let pool = SqlitePool::new("sqlite:%3Amemory:").await?;

    let schema = "
        DROP TABLE IF EXISTS users;
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            email TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL
        );
    ";
    sqlx::query(schema).execute(&pool).await?;
    match User::new("bob@example.com".to_string(), "12345678".to_string(), "12345678".to_string()) {
        Ok(query) => query.execute(&pool).await?,
        Err(e) => panic!("{:?}", e)
    };

    let user = User::find_by_email("bob@example.com".to_string()).fetch_one(&pool).await?;

    let matches = argon2::verify_encoded(&user.password, "12345678".as_bytes()).unwrap();
    println!("Hello, world! {:?} {:?}", matches, user);

    // Session Stuff
    let store = async_sqlx_session::SqliteSessionStore::from_client(pool);
    store.migrate().await.unwrap();

    let mut session = async_session::Session::new();
    session.insert("user_id", user.id).unwrap();

    let cookie_value = store.store_session(session).await.unwrap().unwrap();

    let session = store.load_session(cookie_value.clone()).await.unwrap().unwrap();
    println!("{:?} {:?}", cookie_value, session.get::<usize>("user_id").unwrap());

    Ok(())
}

// Login:
// - find_by_email
// - verify_encoded
// - Session::new

// Logout:
// - store.destroy_session

// Register:
// - User::new
// - Session::new

// Auth:
// - store.load_session
