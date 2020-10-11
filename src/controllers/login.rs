use sqlx::prelude::*;
use tide::prelude::*;
use tide::Redirect;

use crate::AppState;
use crate::models::User;
use crate::templates::LoginTemplate;
use crate::templates::LoginError;

#[derive(Debug, Deserialize)]
struct PostForm {
    email: String,
    password: String,
}

pub async fn get(request: tide::Request<AppState>) -> tide::Result {
    let user_id: i64 = request.session().get("user_id").unwrap_or_default();
    if user_id != 0 {
        return Ok(Redirect::new("/").into());
    }

    Ok(LoginTemplate::new().into())
}

pub async fn post(mut request: tide::Request<AppState>) -> tide::Result {
    let form: PostForm = request.body_form().await?;
    let db = &request.state().db;

    let user = User::find_by_email(form.email).fetch_one(db).await.unwrap();

    let password_matches = argon2::verify_encoded(&user.password, form.password.as_bytes()).unwrap();
    if !password_matches {
        return Ok(LoginTemplate::with_error(LoginError::InvalidCredentials).into());
    }

    let session = request.session_mut();
    session.insert("user_id", user.id).unwrap();

    Ok(Redirect::new("/").into())
}
