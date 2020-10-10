use sqlx::prelude::*;
use tide::prelude::*;
use tide::Redirect;

use crate::AppState;
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

    User::new(
        form.email.clone(), 
        form.password, 
        form.password_confirmation
    ).unwrap().execute(db).await.unwrap();
    let user = User::find_by_email(form.email).fetch_one(db).await.unwrap();

    let session = request.session_mut();
    session.insert("user_id", user.id).unwrap();

    Ok(Redirect::new("/").into())
}
