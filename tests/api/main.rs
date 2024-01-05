//! In these tests, there can be too many postgres connections if
//! you run unit tests in parallel. So run them with `cargo test -- --test-threads=1`

mod conversation;
mod health_check;
mod helpers;
mod login;
mod signup;
mod upload_song;
