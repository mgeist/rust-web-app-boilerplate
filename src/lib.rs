use async_sqlx_session::SqliteSessionStore;
use sqlx::Error as sqlxError;
use sqlx::sqlite::SqlitePool;

pub mod models;

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
