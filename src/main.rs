use std::io::Error as stdError;

use async_session::SessionStore;
use async_sqlx_session::SqliteSessionStore;
use sqlx::prelude::*;
use sqlx::sqlite::SqlitePool;

use lib::{init_app, init_db, init_store};
use lib::models::{PasswordResetToken, User};

#[async_std::main]
async fn main() -> Result<(), stdError> {
    // DB Stuff
    let pool = init_db().await.unwrap();
    // Session Stuff
    let store = init_store(pool.clone()).await.unwrap();
    // Web Stuff
    let app = init_app(pool, store).await;
    app.listen("127.0.0.1:8080").await?;

    // Temp test auth stuff
    // let cookie = register(
    //     &pool, &store, "bob@example.com".to_string(), "12345678".to_string(), "12345678".to_string()
    // ).await;
    // logout(&store, cookie).await;
    // let cookie = login(&pool, &store, "bob@example.com".to_string(), "12345678".to_string()).await;
    // let user = auth(&pool, &store, cookie).await;
    // let token = forgot_password(&pool, user.email).await;
    // reset_password(&pool, token, "87654321".to_string(), "87654321".to_string()).await;


    Ok(())
}

async fn auth(pool: &SqlitePool, store: &SqliteSessionStore, cookie_value: String) -> User {
    let session = (*store).load_session(cookie_value).await.unwrap().unwrap();
    let user_id = session.get("user_id").unwrap();
    let user = User::find_by_id(user_id).fetch_one(pool).await.unwrap();
    println!("Authed {:?}", user);
    return user
}

async fn forgot_password(pool: &SqlitePool, email: String) -> String {
    let user = User::find_by_email(email).fetch_one(pool).await.unwrap();
    PasswordResetToken::new(user.id).unwrap().execute(pool).await.unwrap();
    let reset_token = PasswordResetToken::find_by_user_id(user.id).fetch_one(pool).await.unwrap();
    println!("Created token {:?}", reset_token.token);
    return reset_token.token
}

async fn reset_password(pool: &SqlitePool, token: String, password: String, password_confirmation: String) {
    let mut tx = pool.begin().await.unwrap();

    // TODO check if expired token

    let reset_token = PasswordResetToken::find_by_token(token).fetch_one(&mut tx).await.unwrap();
    let user = User::find_by_id(reset_token.user_id).fetch_one(&mut tx).await.unwrap();
    user.reset_password(password, password_confirmation).unwrap().execute(&mut tx).await.unwrap();
    reset_token.delete().execute(&mut tx).await.unwrap();
    let user = User::find_by_id(reset_token.user_id).fetch_one(&mut tx).await.unwrap();

    tx.commit().await.unwrap();
    println!("Reset password {:?}", user.password);
}

#[cfg(test)]
mod tests {
    #[test]
    fn foo() {
        assert_eq!(2 + 2, 4);
    }

    // #[test]
    // fn bar() {
    //     panic!("This test fails");
    // }
}
