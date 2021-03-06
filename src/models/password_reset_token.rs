use rand::prelude::*;
use rand::distributions::Alphanumeric;
use rand::thread_rng;
use sqlx::FromRow;

use crate::error::Error;
use super::{Query, QueryAs};

#[derive(Debug, FromRow)]
pub struct PasswordResetToken {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
    pub expiration: i32,
}

impl PasswordResetToken {
    pub fn new(user_id: i32) -> Result<Query, Error> {
        let mut rng = thread_rng();
        let chars: String = std::iter::repeat(())
            .map(|()| rng.sample(Alphanumeric))
            .take(64)
            .collect();

        let query = sqlx::query("
            INSERT INTO password_reset_tokens (user_id, token, expiration)
            VALUES ($1, $2, EXTRACT(epoch FROM NOW() + INTERVAL '7' DAY))
        ")
            .bind(user_id)
            .bind(chars);

        Ok(query)
    }

    pub fn find_by_user_id(user_id: i32) -> QueryAs<Self> {
        sqlx::query_as("SELECT * FROM password_reset_tokens WHERE user_id = $1")
            .bind(user_id)
    }

    pub fn find_by_token(token: String) -> QueryAs<Self> {
        sqlx::query_as("SELECT * FROM password_reset_tokens WHERE token = $1")
            .bind(token)
    }

    pub fn delete(&self) -> Query {
        sqlx::query("DELETE FROM password_reset_tokens WHERE id = $1")
            .bind(self.id)
    }
}
