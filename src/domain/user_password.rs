//! src/domain/user_name.rs

use anyhow::Context;
use secrecy::Secret;

/// This type guarantees us that `UserName` is properly formed.
pub struct UserPassword(Secret<String>);

impl UserPassword {
    /// Returns an instance of `UserName` if the input satisfies
    /// our validation constraints on subscriber names.
    pub fn parse(
        input: &str,
        user_inputs: &[&str],
    ) -> Result<UserPassword, anyhow::Error> {
        let is_empty_or_whitespace = input.trim().is_empty();
        let is_too_long = input.chars().count() > 32;
        let is_too_short = input.chars().count() < 8;

        let forbidden_characters =
            ['/', '"', '<', '>', '\\', '{', '}', ';', ':'];

        let contains_forbidden_chars =
            input.chars().any(|g| forbidden_characters.contains(&g));

        if is_empty_or_whitespace {
            Err(anyhow::anyhow!("String is emtpy"))
        } else if is_too_long {
            Err(anyhow::anyhow!("String is too long"))
        } else if is_too_short {
            Err(anyhow::anyhow!(
                "String is too short, should be longer than 2 symbols"
            ))
        } else if contains_forbidden_chars {
            Err(anyhow::anyhow!("String contains forbidden chars"))
        } else {
            let entropy = zxcvbn::zxcvbn(input, user_inputs)
                .context("Failed to compute password entropy")?;
            if entropy.score() < 4 {
                Err(anyhow::anyhow!("Too weak password"))
            } else {
                Ok(UserPassword(Secret::new(input.to_string())))
            }
        }
    }
}

impl AsRef<Secret<String>> for UserPassword {
    fn as_ref(&self) -> &Secret<String> {
        &self.0
    }
}

// ───── Unit tests ───────────────────────────────────────────────────────── //

#[cfg(test)]
mod tests {
    use super::UserPassword;
    #[test]
    fn password_empty() {
        let password = "";
        let result = UserPassword::parse(password, &[]);
        assert!(result.is_err());
    }

    #[test]
    fn password_too_long() {
        let password = "a".repeat(33);
        let result = UserPassword::parse(&password, &[]);
        assert!(result.is_err());
    }

    #[test]
    fn password_too_short() {
        let password = "a".repeat(7);
        let result = UserPassword::parse(&password, &[]);
        assert!(result.is_err());
    }

    #[test]
    fn password_contains_forbidden_characters() {
        let passwords = vec![
            "password/",
            "password\"",
            "password<",
            "password>",
            "password\\",
            "password{",
            "password}",
            "password;",
            "password:",
        ];
        for password in passwords {
            let result = UserPassword::parse(password, &[]);
            assert!(result.is_err());
        }
    }

    #[test]
    fn password_weak() {
        let password = "weak";
        let user_inputs = ["username", "email@example.com"];
        let result = UserPassword::parse(&password, &user_inputs);
        assert!(result.is_err());
    }

    #[test]
    fn password_strong() {
        let password = "Th1s1s@Str0ngP@55w0rd!";
        let user_inputs = ["username", "email@example.com"];
        let result = UserPassword::parse(&password, &user_inputs);
        assert!(result.is_ok());
    }
}
