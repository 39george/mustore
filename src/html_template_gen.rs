use askama::Template;

#[derive(Template)]
// escape = "none": override the template's extension used
// for the purpose of determining the escaper for this template.
// {{ "Escape <>&"|e }} with escape will be this: Escape &lt;&gt;&amp;
// So we disable this
#[template(path = "email_verification.html", escape = "none")]
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
