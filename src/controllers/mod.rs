use sqlx::prelude::*;
use tide::Redirect;

use crate::AppState;
use crate::models::User;
use crate::templates::HelloTemplate;

pub mod register;

pub async fn auth_controller(_request: tide::Request<AppState>) -> tide::Result {
    // let db = &request.state().db;
    // let user_id: i64 = request.session().get("user_id").unwrap_or_default();
    // let user = User::find_by_id(user_id).fetch_one(db).await.unwrap();

    Ok(Redirect::new("/").into())
}

pub async fn hello_controller(request: tide::Request<AppState>) -> tide::Result {
    let db = &request.state().db;
    let user_id: i64 = request.session().get("user_id").unwrap_or_default();
    let mut name = "guest".to_string();
    if user_id != 0 {
        name = User::find_by_id(user_id).fetch_one(db).await.unwrap().email;
    }
    Ok(HelloTemplate::new(&name).into())
}

pub async fn login_controller(mut request: tide::Request<AppState>) -> tide::Result {
    let db = &request.state().db;
    let email = "bob@example.com".to_string();
    let password = "123123123".to_string();

    let user = User::find_by_email(email).fetch_one(db).await.unwrap();
    let password_matches = argon2::verify_encoded(&user.password, password.as_bytes()).unwrap();

    if password_matches {
        let session = request.session_mut();
        session.insert("user_id", user.id).unwrap();
    }

    Ok(Redirect::new("/").into())
}

pub async fn logout_controller(mut request: tide::Request<AppState>) -> tide::Result {
    request.session_mut().destroy();

    Ok(Redirect::new("/").into())
}
