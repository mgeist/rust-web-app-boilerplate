use std::io::Error as stdError;

use async_session::SessionStore;
use async_sqlx_session::SqliteSessionStore;
use sqlx::prelude::*;
use sqlx::sqlite::SqlitePool;
use tide::Redirect;
use tide::sessions::SessionMiddleware;

use lib::{AppState, init_db, init_store};
use lib::models::{PasswordResetToken, User};
use lib::templates::HelloTemplate;

#[async_std::main]
async fn main() -> Result<(), stdError> {
    // DB Stuff
    let pool = init_db().await.unwrap();
    // Session Stuff
    let store = init_store(pool.clone()).await.unwrap();

    // Temp test auth stuff
    // let cookie = register(
    //     &pool, &store, "bob@example.com".to_string(), "12345678".to_string(), "12345678".to_string()
    // ).await;
    // logout(&store, cookie).await;
    // let cookie = login(&pool, &store, "bob@example.com".to_string(), "12345678".to_string()).await;
    // let user = auth(&pool, &store, cookie).await;
    // let token = forgot_password(&pool, user.email).await;
    // reset_password(&pool, token, "87654321".to_string(), "87654321".to_string()).await;

    // Web Stuff
    tide::log::start();

    let session_secret = std::env::var("SECRET_KEY").unwrap();
    let session_middleware = SessionMiddleware::new(store, session_secret.as_bytes());

    let mut app = tide::with_state(AppState { db: pool.clone() });
    app.with(session_middleware);

    app.at("/").get(hello);
    app.at("/register").get(register);
    app.listen("127.0.0.1:8080").await?;

    Ok(())
}

pub async fn hello(request: tide::Request<AppState>) -> tide::Result {
    let user_id: i64 = request.session().get("user_id").unwrap_or_default();
    let mut name = "Bob".to_string();
    if user_id != 0 {
        name = user_id.to_string();
    }
    Ok(HelloTemplate::new(&name).into())
}

pub async fn register(mut request: tide::Request<AppState>) -> tide::Result {
    let db = &request.state().db;
    let email = "bob@example.com".to_string();
    let password = "123123123".to_string();

    User::new(email.clone(), password.clone(), password).unwrap().execute(db).await.unwrap();
    let user = User::find_by_email(email).fetch_one(db).await.unwrap();

    let session = request.session_mut();
    session.insert("user_id", user.id).unwrap();

    Ok(Redirect::new("/").into())
}

async fn login(pool: &SqlitePool, store: &SqliteSessionStore, email: String, password: String) -> String {
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
    reset_token.delete().unwrap().execute(&mut tx).await.unwrap();
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
