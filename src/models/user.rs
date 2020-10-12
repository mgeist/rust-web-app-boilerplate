use argon2::{Config, hash_encoded, verify_encoded};
use rand::prelude::*;
use rand::thread_rng;
use sqlx::FromRow;

use crate::error::{Error, ErrorKind};
use super::{Query, QueryAs};

#[derive(Debug, FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub created: i32,
    pub updated: i32,
}

impl User {
    pub fn new(email: String, password: String, password_confirmation: String) -> Result<Query, Error> {
        let hash = generate_password(password, password_confirmation)?;
        let query = sqlx::query("
            INSERT INTO users (email, password, created, updated)
            VALUES ($1, $2, EXTRACT(epoch FROM NOW()), EXTRACT(epoch FROM NOW()))
        ")
            .bind(email)
            .bind(hash);
        Ok(query)
    }

    pub fn validate_password(password: String, password_confirmation: String) -> Result<(), Error> {
        if password.len() < 8 {
            return Err(Error::new(ErrorKind::PasswordTooShort));
        } else if password.len() > 64 {
            return Err(Error::new(ErrorKind::PasswordTooLong));
        } else if !password.eq(&password_confirmation) {
            return Err(Error::new(ErrorKind::PasswordMismatch));
        } else {
            return Ok(());
        }
    }

    pub fn all() -> QueryAs<Self> {
        sqlx::query_as("SELECT * FROM users")
    }

    pub fn find_by_id(id: i32) -> QueryAs<Self> {
        sqlx::query_as("SELECT * FROM users WHERE id = $1")
            .bind(id)
    }

    pub fn find_by_email(email: String) -> QueryAs<Self> {
        sqlx::query_as("SELECT * FROM users WHERE email = $1")
            .bind(email)
    }

    pub fn reset_password(&self, password: String, password_confirmation: String) -> Result<Query, Error> {
        let matches;
        match verify_encoded(&self.password, password.as_bytes()) {
            Ok(m) => matches = m,
            Err(_e) => return Err(Error::new(ErrorKind::UnknownError))
        }
        if matches { return Err(Error::new(ErrorKind::PasswordMismatch)) }

        let hash = generate_password(password, password_confirmation)?;

        let query = sqlx::query("UPDATE users SET password = $1, updated = EXTRACT(epoch FROM NOW()) WHERE id = $2")
            .bind(hash)
            .bind(self.id);
        Ok(query)
    }
}

fn generate_password(password: String, password_confirmation: String) -> Result<String, Error> {
    User::validate_password(password.clone(), password_confirmation)?;

    let mut salt = [0u8; 16];
    thread_rng().fill_bytes(&mut salt);

    let hash;
    match hash_encoded(password.as_bytes(), &salt, &Config::default()) {
        Ok(h) => hash = h,
        Err(_e) => return Err(Error::new(ErrorKind::UnknownError)),
    };
    Ok(hash)
}
