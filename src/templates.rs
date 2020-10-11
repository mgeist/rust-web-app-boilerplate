use askama::Template;

#[derive(Template)]
#[template(path = "hello.html")]
pub struct HelloTemplate<'a> {
    name: &'a str,
}

impl<'a> HelloTemplate<'a> {
    pub fn new(name: &'a str) -> Self {
        Self { name }
    }
}

#[derive(Template)]
#[template(path = "register.html")]
pub struct RegisterTemplate {}
impl RegisterTemplate {
    pub fn new() -> Self {
        return Self {}
    }
}

pub enum LoginError {
    InvalidCredentials
}
#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    error: Option<LoginError>,
}

impl LoginTemplate {
    pub fn new() -> Self {
        return Self { 
            error: None
        }
    }

    pub fn with_error(error: LoginError) -> Self {
        return Self {
            error: Some(error)
        }
    }
}
