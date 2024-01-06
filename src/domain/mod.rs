//! src/domain/mod.rs

use validator::ValidationError;

pub mod music_parameters;
pub mod requests;
pub mod responses;
pub mod signup_token;
pub mod user_candidate;
pub mod user_email;
pub mod user_name;
pub mod user_password;
pub mod user_role;

pub fn forbidden_characters(input: &str) -> Result<(), ValidationError> {
    let forbidden_characters =
        ['/', '(', ')', '"', '<', '>', '\\', '{', '}', ';', ':'];

    if input.chars().any(|g| forbidden_characters.contains(&g)) {
        Err(ValidationError::new("String contains forbidden chars"))
    } else {
        Ok(())
    }
}
