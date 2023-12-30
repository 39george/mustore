//! src/domain/mod.rs

use validator::ValidationError;

pub mod music_parameters;
pub mod requests;
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

pub fn validate_slice_bounds_characters(
    values: &[String],
    (min, max): (usize, usize),
) -> Result<(), ValidationError> {
    for val in values.iter() {
        crate::domain::forbidden_characters(val)?;
        if val.len() < min {
            return Err(ValidationError::new("Value is too short"));
        } else if val.len() > max {
            return Err(ValidationError::new("Value is too long"));
        }
    }
    Ok(())
}
