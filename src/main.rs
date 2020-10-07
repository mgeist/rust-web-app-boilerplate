use async_session::SessionStore;
use async_sqlx_session::SqliteSessionStore;
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

    // Session Stuff
    let store = SqliteSessionStore::from_client(pool.clone());
    store.migrate().await.unwrap();

    let cookie = register(
        &pool, &store, "bob@example.com".to_string(), "12345678".to_string(), "12345678".to_string()
    ).await;

    logout(&store, cookie).await;

    let cookie = login(&pool, &store, "bob@example.com".to_string(), "12345678".to_string()).await;

    auth(&pool, &store, cookie).await;
    Ok(())
}

async fn register(pool: &SqlitePool, store: &SqliteSessionStore, email: String, password: String, password_confirmation: String) -> String {
    User::new(email.clone(), password, password_confirmation).unwrap().execute(pool).await.unwrap();
    let user = User::find_by_email(email).fetch_one(pool).await.unwrap();

    let mut session = async_session::Session::new();
    session.insert("user_id", user.id).unwrap();

    let cookie_value = (*store).store_session(session).await.unwrap().unwrap();
    println!("Registered, user {:?}, cookie {:?}", user, cookie_value);
    return cookie_value
}

async fn login(pool: &SqlitePool, store: &SqliteSessionStore, email: String, password: String) -> String{
    let user = User::find_by_email(email).fetch_one(pool).await.unwrap();

    let matches = argon2::verify_encoded(&user.password, password.as_bytes()).unwrap();
    if !matches { return "".to_string() }

    let mut session = async_session::Session::new();
    session.insert("user_id", user.id).unwrap();

    let cookie_value = (*store).store_session(session).await.unwrap().unwrap();
    println!("Logged in, cookie {:?}", cookie_value);
    return cookie_value
}

async fn logout(store: &SqliteSessionStore, cookie_value: String) {
    let session = (*store).load_session(cookie_value).await.unwrap().unwrap();
    store.destroy_session(session).await.unwrap();
    println!("Logged out");
}

async fn auth(pool: &SqlitePool, store: &SqliteSessionStore, cookie_value: String) -> User {
    let session = (*store).load_session(cookie_value).await.unwrap().unwrap();
    let user_id = session.get("user_id").unwrap();
    let user = User::find_by_id(user_id).fetch_one(pool).await.unwrap();
    println!("Authed {:?}", user);
    return user
}

// Forgot Password
// Reset Password
