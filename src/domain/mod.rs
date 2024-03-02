//! src/domain/mod.rs

pub mod general_types;
pub mod music_parameters;
pub mod object_key;
pub mod requests;
pub mod responses;
pub mod signup_token;
pub mod upload_request;
pub mod user_candidate;
pub mod user_email;
pub mod user_name;
pub mod user_password;
pub mod user_role;

pub const MIN_FILENAME_LEN: usize = 2;
pub const MAX_FILENAME_LEN: usize = 50;
pub const MIN_MESSAGE_LEN: usize = 1;
pub const MAX_MESSAGE_LEN: usize = 2500;

pub const FORBIDDEN_CHARS: [char; 11] =
    ['/', '(', ')', '"', '<', '>', '\\', '{', '}', ';', ':'];
pub const OBJ_KEY_MIN_LEN: usize = 10;
pub const OBJ_KEY_MAX_LEN: usize = 500;
pub const PRDCT_NAME_MIN_LEN: usize = 2;
pub const PRDCT_NAME_MAX_LEN: usize = 30;
pub const PRDCT_DESC_MIN_LEN: usize = 15;
pub const PRDCT_DESC_MAX_LEN: usize = 400;
pub const MOOD_MIN_LEN: usize = 2;
pub const MOOD_MAX_LEN: usize = 50;
pub const GENRE_MIN_LEN: usize = 2;
pub const GENRE_MAX_LEN: usize = 50;
pub const MIN_TEMPO: i16 = 40;
pub const MAX_TEMPO: i16 = 320;
pub const MIN_AUDIO_DURATION_SEC: i16 = 15;
pub const MAX_AUDIO_DURATION_SEC: i16 = 600;
pub const MIN_LYRIC_LEN: usize = 1;
pub const MAX_LYRIC_LEN: usize = 5000;
pub const MIN_LYRIC_COUNT: usize = 1;
pub const MAX_LYRIC_COUNT: usize = 5;

pub const MAX_ATTACHMENTS_COUNT: usize = 10;

pub fn forbidden_characters(input: &str, _: &()) -> garde::Result {
    if input.chars().any(|g| FORBIDDEN_CHARS.contains(&g)) {
        Err(garde::Error::new("String contains forbidden chars"))
    } else {
        Ok(())
    }
}

pub fn contains_no_control_characters(s: &str, _: &()) -> garde::Result {
    if s.chars().any(|c| c.is_control()) {
        Err(garde::Error::new("String contains control characters"))
    } else {
        Ok(())
    }
}
