//! src/domain/user_email.rs

/// This type guarantees correctness of `subscriber's` email address.
#[derive(Clone, Debug)]
pub struct UserEmail(String);

impl UserEmail {
    pub fn parse(email: &str) -> Result<Self, anyhow::Error> {
        if garde::rules::email::parse_email(email).is_ok() {
            Ok(Self(email.to_string()))
        } else {
            Err(anyhow::anyhow!("Is not a valid subscriber email."))
        }
    }
}

impl AsRef<str> for UserEmail {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for UserEmail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::UserEmail;
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;

    #[test]
    fn valied_emails_are_parsed_successfully() {
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let valid_email: String = SafeEmail().fake_with_rng(&mut rng);
            assert!(UserEmail::parse(&valid_email).is_ok());
        }
    }

    #[test]
    fn empty_string_is_rejected() {
        let email = "".to_string();
        assert!(UserEmail::parse(&email).is_err())
    }

    #[test]
    fn email_missing_at_symbol_is_rejected() {
        let email = "ursuladomail.com".to_string();
        assert!(UserEmail::parse(&email).is_err())
    }

    #[test]
    fn email_missing_subject_is_rejected() {
        let email = "@domail.com".to_string();
        assert!(UserEmail::parse(&email).is_err())
    }

    #[test]
    fn valid_emails_are_parsed_successfully() {
        let email: String = SafeEmail().fake();
        assert!(UserEmail::parse(&email).is_ok())
    }
}
