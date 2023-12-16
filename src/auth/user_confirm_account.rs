use crate::cornucopia::queries::user_auth_queries;
use anyhow::Context;
use axum::extract::Query;
use axum::extract::State;
use http::StatusCode;
use identicon_rs::Identicon;
use redis::AsyncCommands;
use serde::Deserialize;
use std::collections::HashMap;
use std::io::Cursor;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use super::AuthError;
use super::UserCandidate;
use crate::startup::AppState;
use crate::telemetry::spawn_blocking_with_tracing;

// ───── Types ────────────────────────────────────────────────────────────── //

#[derive(Deserialize)]
pub struct UserConfirmationQuery {
    email: String,
    token: String,
}

// ───── Handlers ─────────────────────────────────────────────────────────── //

#[tracing::instrument(name = "Account confirmation", skip_all)]
pub async fn confirm(
    State(app_state): State<AppState>,
    Query(UserConfirmationQuery { email, token }): Query<UserConfirmationQuery>,
) -> Result<StatusCode, AuthError> {
    let mut db_client = app_state
        .pg_pool
        .get()
        .await
        .context("Failed to get connection from postgres pool")
        .map_err(AuthError::AccountConfirmationFailed)?;

    let mut redis_client = app_state
        .redis_pool
        .get()
        .await
        .context("Failed to get redis connection from pool")
        .map_err(AuthError::InternalError)?;

    let user_candidate_data =
        get_user_candidate_data(&mut redis_client, &email)
            .await
            .context("Failed to get data from redis")
            .map_err(AuthError::AccountConfirmationFailed)?;

    if user_candidate_data.validation_token != token {
        return Err(AuthError::AccountConfirmationFailed(anyhow::anyhow!(
            "Token is different!"
        )));
    }

    let email_c = email.clone();
    let identicon = spawn_blocking_with_tracing(move || {
        generate_identicon_png(&email_c)
            .context("Failed to generate identicon")
            .map_err(AuthError::InternalError)
    })
    .await
    .context("Failed to join tokio thread handle")??;

    let transaction = db_client
        .transaction()
        .await
        .context("Failed to get a transaction from pg")
        .map_err(AuthError::InternalError)?;

    let user_settings_id = user_auth_queries::insert_new_user_settings()
        .bind(&transaction)
        .one()
        .await
        .context("Failed to insert user_settings")
        .map_err(AuthError::InternalError)?;

    // Upload identicon to the object storage
    let avatar_filename = format!("avatar_{}.png", &email);
    let avatar_uri = app_state
        .object_storage
        .put(&avatar_filename, identicon)
        .await?;

    if let Err(e) = user_auth_queries::insert_new_user()
        .bind(
            &transaction,
            &user_settings_id,
            &user_candidate_data.username,
            &avatar_uri,
            &user_candidate_data.email,
            &user_candidate_data.password_hash,
            &user_candidate_data.role.into(),
        )
        .await
        .context("Failed to insert a new user to the pg")
        .map_err(AuthError::AccountConfirmationFailed)
    {
        app_state
            .object_storage
            .delete_object_by_uri(&avatar_uri)
            .await?;
        transaction
            .rollback()
            .await
            .context("Failed to rollback pg transaction")
            .map_err(AuthError::InternalError)?;
        return Err(e);
    }

    if let Err(e) = transaction
        .commit()
        .await
        .context("Failed to commit a pg transaction")
        .map_err(AuthError::InternalError)
    {
        app_state
            .object_storage
            .delete_object_by_uri(&avatar_uri)
            .await?;

        return Err(e);
    }

    Ok(StatusCode::OK)
}

#[tracing::instrument(name = "Get user candidate data from redis", skip_all)]
async fn get_user_candidate_data(
    con: &mut redis::aio::Connection,
    user_email: &str,
) -> redis::RedisResult<UserCandidate> {
    let key = format!("user_candidate:{}", user_email);
    let result: HashMap<String, String> = con.hgetall(&key).await?;
    con.del(&key).await?;
    UserCandidate::from_map(result)
}

#[tracing::instrument(name = "Generate identicon")]
fn generate_identicon_png(from_str: &str) -> Result<Vec<u8>, anyhow::Error> {
    let image = Identicon::new(from_str)
        .set_border(0)
        .generate_image()
        .context("Failed to generage image from string: {from_str}")?;

    let mut image_bytes: Vec<u8> = Vec::new();
    image
        .write_to(
            &mut Cursor::new(&mut image_bytes),
            image::ImageOutputFormat::Png,
        )
        .context("Failed to write DynamicImage into Vec<u8>")?;
    Ok(image_bytes)
}
