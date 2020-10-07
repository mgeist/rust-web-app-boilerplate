use rand::prelude::*;
use rand::distributions::Alphanumeric;
use rand::{thread_rng};
use sqlx::{FromRow, Sqlite};

// TODO sort out this stuff
#[derive(Debug)]
pub enum Error {
    ValidationError
}
type Query = sqlx::Query<'static, Sqlite>;
type QueryAs<T> = sqlx::QueryAs<'static, Sqlite, T>;

#[derive(Debug, FromRow)]
pub struct PasswordResetToken {
    pub id: i64,
    pub user_id: i64,
    pub token: String,
    // TODO: expiration_date
}

impl PasswordResetToken {
    pub fn new(user_id: i64) -> Result<Query, Error> {
        let mut rng = thread_rng();
        let chars: String = std::iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .take(64)
            .collect();

        let query = sqlx::query("INSERT INTO password_reset_tokens (user_id, token) VALUES ($1, $2)")
            .bind(user_id)
            .bind(chars);

        Ok(query)
    }

    pub fn find_by_user_id(user_id: i64) -> QueryAs<Self> {
        sqlx::query_as("SELECT * FROM password_reset_tokens WHERE user_id = ?")
            .bind(user_id)
    }

    pub fn find_by_token(token: String) -> QueryAs<Self> {
        sqlx::query_as("SELECT * FROM password_reset_tokens WHERE token = ?")
            .bind(token)
    }
}
