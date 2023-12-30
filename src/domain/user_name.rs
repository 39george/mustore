//! src/domain/user_name.rs

use crate::domain::forbidden_characters;

/// This type guarantees us that `UserName` is properly formed.
pub struct UserName(String);

impl UserName {
    /// Returns an instance of `UserName` if the input satisfies
    /// our validation constraints on subscriber names.
    pub fn parse(name: &str) -> Result<UserName, anyhow::Error> {
        let is_empty_or_whitespace = name.trim().is_empty();
        let is_too_long = name.chars().count() > 256;
        let is_too_short = name.chars().count() < 3;
        let contains_forbidden_chars = forbidden_characters(name).is_err();

        if is_empty_or_whitespace {
            Err(anyhow::anyhow!("String is emtpy"))
        } else if is_too_long {
            Err(anyhow::anyhow!("String is too long"))
        } else if is_too_short {
            Err(anyhow::anyhow!(
                "Username is too short, should be longer than 2 symbols: {}",
                name
            ))
        } else if contains_forbidden_chars {
            Err(anyhow::anyhow!("String contains forbidden chars"))
        } else {
            Ok(UserName(name.to_string()))
        }
    }
}

impl AsRef<str> for UserName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

// ───── Unit tests ───────────────────────────────────────────────────────── //

#[cfg(test)]
mod tests {
    use super::UserName;

    #[test]
    fn a_256_char_long_name_is_valid() {
        let name = "a".repeat(256);
        assert!(UserName::parse(&name).is_ok());
    }

    #[test]
    fn a_name_longer_than_256_chars_is_rejected() {
        let name = "a".repeat(257);
        assert!(UserName::parse(&name).is_err());
    }

    #[test]
    fn a_name_shorter_than_3_chars_is_rejected() {
        let name = "ab";
        assert!(UserName::parse(&name).is_err());
    }

    #[test]
    fn whitespace_only_names_are_rejected() {
        let name = " ".to_string();
        assert!(UserName::parse(&name).is_err());
    }

    #[test]
    fn emtpy_string_is_rejected() {
        let name = "".to_string();
        assert!(UserName::parse(&name).is_err());
    }

    #[test]
    fn names_contanining_an_invalid_character_are_rejected() {
        for name in &['/', '(', ')', '"', '<', '>', '\\', '{', '}'] {
            let name = name.to_string();
            assert!(UserName::parse(&name).is_err());
        }
    }

    #[test]
    fn a_valid_name_is_parsed_successfully() {
        let name = "Ursula Le Guin".to_string();
        assert!(UserName::parse(&name).is_ok());
    }
}
