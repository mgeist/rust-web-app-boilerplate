use async_sqlx_session::PostgresSessionStore;
use sqlx::Error as sqlxError;
use sqlx::postgres::PgPool;
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
    pub db: PgPool,
}

pub async fn init_db() -> Result<PgPool, sqlxError> {
    let db_url = std::env::var("DATABASE_URL").unwrap();
    let pool = PgPool::new(&db_url).await?;
    
    sqlx::query("SELECT 1").execute(&pool).await?;
    Ok(pool)
}

pub async fn init_store(pool: PgPool) -> Result<PostgresSessionStore, sqlxError> {
    let store = PostgresSessionStore::from_client(pool);
    store.migrate().await?;

    Ok(store)
}

pub async fn init_app(pool: PgPool, store: PostgresSessionStore) -> tide::Server<AppState> {
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
