use sqlx::prelude::*;
use tide::prelude::*;
use tide::Redirect;

use crate::AppState;
use crate::error::{Error, ErrorKind};
use crate::models::User;
use crate::templates::LoginTemplate;

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

    let user;
    match User::find_by_email(form.email).fetch_one(db).await {
        Ok(u) => user = u,
        Err(_e) => return Ok(LoginTemplate::with_error(Error::new(ErrorKind::InvalidCredentials)).into()),
    }

    let password_matches;
    match argon2::verify_encoded(&user.password, form.password.as_bytes()) {
        Ok(p) => password_matches = p,
        Err(_e) => return Ok(LoginTemplate::with_error(Error::new(ErrorKind::InvalidCredentials)).into()),
    }

    if !password_matches {
        return Ok(LoginTemplate::with_error(Error::new(ErrorKind::InvalidCredentials)).into());
    }

    let session = request.session_mut();
    if let Err(_e) = session.insert("user_id", user.id) {
        return Ok(LoginTemplate::with_error(Error::new(ErrorKind::UnknownError)).into());
    }

    Ok(Redirect::new("/").into())
}
