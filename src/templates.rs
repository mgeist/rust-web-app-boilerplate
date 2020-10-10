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
