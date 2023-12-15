use askama::Template;

#[derive(Template)]
#[template(path = "email_verification.html")]
pub struct VerifyEmailTemplate<'a> {
    name: &'a str,
    link: &'a str,
}

impl<'a> VerifyEmailTemplate<'a> {
    pub fn new(name: &'a str, reference: &'a str) -> Self {
        VerifyEmailTemplate {
            name,
            link: reference,
        }
    }
}
