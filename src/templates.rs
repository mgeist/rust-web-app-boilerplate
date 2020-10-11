use askama::Template;

use crate::error::Error;

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
pub struct RegisterTemplate {
    error: Option<Error>,
}
impl RegisterTemplate {
    pub fn new() -> Self {
        return Self {
            error: None
        }
    }

    pub fn with_error(error: Error) -> Self {
        return Self {
            error: Some(error)
        }
    }
}

#[derive(Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    error: Option<Error>,
}

impl LoginTemplate {
    pub fn new() -> Self {
        return Self { 
            error: None
        }
    }

    pub fn with_error(error: Error) -> Self {
        return Self {
            error: Some(error)
        }
    }
}
