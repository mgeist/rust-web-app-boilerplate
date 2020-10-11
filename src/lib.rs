use async_sqlx_session::SqliteSessionStore;
use sqlx::Error as sqlxError;
use sqlx::sqlite::SqlitePool;
use tide::sessions::SessionMiddleware;

mod controllers;
mod error;
pub mod models;
pub mod templates;

use controllers::{
    auth_controller,
    hello_controller, 
    logout_controller, 
    login,
    register,
};

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
}

pub async fn init_db() -> Result<SqlitePool, sqlxError> {
    let pool = SqlitePool::new("sqlite:%3Amemory:").await?;

    let schema = "
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            email TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL,
            created INTEGER NOT NULL,
            updated INTEGER NOT NULL
        );

        CREATE TABLE IF NOT EXISTS password_reset_tokens (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            user_id INTEGER NOT NULL UNIQUE,
            token TEXT NOT NULL,
            expiration INTEGER NOT NULL,
            FOREIGN KEY(user_id) REFERENCES users(id)
        );
    ";
    sqlx::query(schema).execute(&pool).await?;
    
    Ok(pool)
}

pub async fn init_store(pool: SqlitePool) -> Result<SqliteSessionStore, sqlxError> {
    let store = SqliteSessionStore::from_client(pool);
    store.migrate().await?;

    Ok(store)
}

pub async fn init_app(pool: SqlitePool, store: SqliteSessionStore) -> tide::Server<AppState> {
    tide::log::start();

    let session_secret = std::env::var("SECRET_KEY").unwrap();
    let session_middleware = SessionMiddleware::new(store, session_secret.as_bytes());

    let mut app = tide::with_state(AppState { db: pool.clone() });
    app.with(session_middleware);

    app.at("/").get(hello_controller);
    app.at("/auth").get(auth_controller);
    app.at("/login")
        .get(login::get)
        .post(login::post);
    app.at("/logout").get(logout_controller);
    app.at("/register")
        .get(register::get)
        .post(register::post);
    app
}
