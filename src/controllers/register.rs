use sqlx::prelude::*;
use tide::prelude::*;
use tide::Redirect;

use crate::AppState;
use crate::error::{Error, ErrorKind};
use crate::models::User;
use crate::templates::RegisterTemplate;

#[derive(Debug, Deserialize)]
struct PostForm {
    email: String,
    password: String,
    password_confirmation: String,
}

pub async fn get(_request: tide::Request<AppState>) -> tide::Result {
    Ok(RegisterTemplate::new().into())
}

pub async fn post(mut request: tide::Request<AppState>) -> tide::Result {
    let form: PostForm = request.body_form().await?;
    let db = &request.state().db;

    if let Err(e) = User::validate_password(form.password.clone(), form.password_confirmation.clone()) {
        return Ok(RegisterTemplate::with_error(e).into());
    }

    let query;
    match User::new(form.email.clone(), form.password, form.password_confirmation) {
        Ok(q) => query = q,
        Err(e) => return Ok(RegisterTemplate::with_error(e).into()),
    }

    let result = query.execute(db).await;
    if result.is_err() {
        return Ok(RegisterTemplate::with_error(Error::new(ErrorKind::UnknownError)).into());
    }

    let user;
    match User::find_by_email(form.email).fetch_one(db).await {
        Ok(u) => user = u,
        Err(_e) => return Ok(RegisterTemplate::with_error(Error::new(ErrorKind::UnknownError)).into()),
    }

    let session = request.session_mut();
    if let Err(_e) = session.insert("user_id", user.id) {
        return Ok(RegisterTemplate::with_error(Error::new(ErrorKind::UnknownError)).into());
    }

    Ok(Redirect::new("/").into())
}
