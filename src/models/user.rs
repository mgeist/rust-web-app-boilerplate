use argon2::{Config, hash_encoded, verify_encoded};
use rand::prelude::*;
use rand::{thread_rng};
use sqlx::{FromRow, Sqlite};

#[derive(Debug)]
pub enum Error {
    ValidationError
}
type Query = sqlx::Query<'static, Sqlite>;
type QueryAs<T> = sqlx::QueryAs<'static, Sqlite, T>;

#[derive(Debug, FromRow)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub password: String,
    pub created: i32,
    pub updated: i32,
}

impl User {
    pub fn new(email: String, password: String, password_confirmation: String) -> Result<Query, Error> {
        let hash = generate_password(password, password_confirmation).unwrap();
        let query = sqlx::query("
            INSERT INTO users (email, password, created, updated)
            VALUES ($1, $2, STRFTIME('%s', 'now'), STRFTIME('%s', 'now'))
        ")
            .bind(email)
            .bind(hash);
        Ok(query)
    }

    pub fn all() -> QueryAs<Self> {
        sqlx::query_as("SELECT * FROM users")
    }

    pub fn find_by_id(id: i64) -> QueryAs<Self> {
        sqlx::query_as("SELECT * FROM users WHERE id = $1")
            .bind(id)
    }

    pub fn find_by_email(email: String) -> QueryAs<Self> {
        sqlx::query_as("SELECT * FROM users WHERE email = $2")
            .bind(email)
    }

    pub fn reset_password(&self, password: String, password_confirmation: String) -> Result<Query, Error> {
        let matches = verify_encoded(&self.password, password.as_bytes()).unwrap();
        if matches { return Err(Error::ValidationError) }

        let hash = generate_password(password, password_confirmation).unwrap();

        let query = sqlx::query("UPDATE users SET password = $1, updated = STRFTIME('%s', 'now') WHERE id = $2")
            .bind(hash)
            .bind(self.id);
        Ok(query)
    }
}

fn generate_password(password: String, password_confirmation: String) -> Result<String, Error> {
    // TODO:
    // - move validations to a validations module
    if !password.eq(&password_confirmation) {
        Err(Error::ValidationError)
    }
    else if password.len() < 8 || password.len() > 64 {
        Err(Error::ValidationError)
    } else {
        let mut salt = [0u8; 16];
        thread_rng().fill_bytes(&mut salt);

        let hash;
        match hash_encoded(password.as_bytes(), &salt, &Config::default()) {
            Ok(h) => hash = h,
            Err(_e) => return Err(Error::ValidationError)
        };
        Ok(hash)
    }
}
