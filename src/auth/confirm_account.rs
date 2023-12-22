use anyhow::Context;
use axum::extract::Query;
use axum::extract::State;
use deadpool_postgres::Transaction;
use fred::clients::RedisPool;
use fred::interfaces::HashesInterface;
use fred::interfaces::KeysInterface;
use fred::interfaces::RedisResult;
use http::StatusCode;
use identicon_rs::Identicon;
use serde::Deserialize;
use std::collections::HashMap;
use std::io::Cursor;

// ───── Current Crate Imports ────────────────────────────────────────────── //

use super::AuthError;
use crate::cornucopia::queries::user_auth_queries;
use crate::domain::user_candidate::UserCandidate;
use crate::startup::AppState;
use crate::telemetry::spawn_blocking_with_tracing;

// ───── Types ────────────────────────────────────────────────────────────── //

#[derive(Deserialize)]
pub struct UserConfirmationQuery {
    email: String,
    token: String,
}

// ───── Handlers ─────────────────────────────────────────────────────────── //

/// If error, we return here only an `AccountConfirmationFailed`
/// to redirect user to a special page, because React app will not handle
/// our `internal error` case.
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

    let user_candidate_data =
        get_user_candidate_data(&app_state.redis_pool, &email)
            .await
            .context("Failed to get user candidate data from redis, it is possible that 30 minutes are over.")
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
            .map_err(AuthError::AccountConfirmationFailed)
    })
    .await
    .context("Failed to join tokio thread handle")??;

    let transaction = db_client
        .transaction()
        .await
        .context("Failed to get a transaction from pg")
        .map_err(AuthError::AccountConfirmationFailed)?;

    let user_settings_id = user_auth_queries::insert_new_user_settings()
        .bind(&transaction)
        .one()
        .await
        .context("Failed to insert user_settings")
        .map_err(AuthError::AccountConfirmationFailed)?;

    // Upload identicon to the object storage
    let avatar_key = format!("avatar_{}.png", &email);
    app_state.object_storage.put(&avatar_key, identicon).await?;

    let user_id = match user_auth_queries::insert_new_user()
        .bind(
            &transaction,
            &user_settings_id,
            &user_candidate_data.username,
            &user_candidate_data.email,
            &user_candidate_data.password_hash,
        )
        .one()
        .await
        .context("Failed to insert a new user to the pg")
        .map_err(AuthError::AccountConfirmationFailed)
    {
        Ok(id) => id,
        Err(e) => {
            app_state
                .object_storage
                .delete_object_by_key(&avatar_key)
                .await?;
            transaction
                .rollback()
                .await
                .context("Failed to rollback pg transaction")
                .map_err(AuthError::AccountConfirmationFailed)?;
            return Err(e);
        }
    };

    if let Err(e) =
        match (user_candidate_data.role, user_candidate_data.admin_token) {
            (None, Some(admin_token)) => {
                if let Err(e) =
                    verify_admin_token(&transaction, admin_token).await
                {
                    Err(e)
                } else {
                    user_auth_queries::store_user_permission()
                        .bind(&transaction, &user_id, &"group.administrators")
                        .await
                        .context(
                            "Failed to insert an admin permission to the pg",
                        )
                        .map_err(AuthError::AccountConfirmationFailed)
                }
            }
            (Some(user_role), None) => {
                user_auth_queries::store_user_permission()
                    .bind(
                        &transaction,
                        &user_id,
                        &user_role.to_permission_string(),
                    )
                    .await
                    .context("Failed to insert an user permission to the pg")
                    .map_err(AuthError::AccountConfirmationFailed)
            }
            _ => {
                return Err(AuthError::SignupFailed(anyhow::anyhow!(
                    "User should have only role, or only admin token!"
                )))
            }
        }
    {
        app_state
            .object_storage
            .delete_object_by_key(&avatar_key)
            .await?;
        transaction
            .rollback()
            .await
            .context("Failed to rollback pg transaction")
            .map_err(AuthError::AccountConfirmationFailed)?;
        return Err(e);
    }

    if let Err(e) = user_auth_queries::insert_user_image()
        .bind(&transaction, &avatar_key, &user_id)
        .await
        .context("Failed to insert a new user to the pg")
        .map_err(AuthError::AccountConfirmationFailed)
    {
        app_state
            .object_storage
            .delete_object_by_key(&avatar_key)
            .await?;
        transaction
            .rollback()
            .await
            .context("Failed to rollback pg transaction")
            .map_err(AuthError::AccountConfirmationFailed)?;
        return Err(e);
    }

    if let Err(e) = transaction
        .commit()
        .await
        .context("Failed to commit a pg transaction")
        .map_err(AuthError::AccountConfirmationFailed)
    {
        app_state
            .object_storage
            .delete_object_by_key(&avatar_key)
            .await?;

        return Err(e);
    }

    Ok(StatusCode::OK)
}

#[tracing::instrument(name = "Get user candidate data from redis", skip_all)]
async fn get_user_candidate_data(
    con: &RedisPool,
    user_email: &str,
) -> RedisResult<UserCandidate> {
    let key = format!("user_candidate:{}", user_email);
    let result: HashMap<String, String> = con.hgetall(&key).await?;
    con.del(&key).await?;
    UserCandidate::try_from(result)
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

#[tracing::instrument(name = "Trying to validate admin token", skip_all)]
async fn verify_admin_token(
    client: &Transaction<'_>,
    admin_token: uuid::Uuid,
) -> Result<(), AuthError> {
    // Token exists
    let token = user_auth_queries::get_admin_signup_token()
        .bind(client, &admin_token)
        .one()
        .await
        .context("Failed to get admin token from pg")
        .map_err(AuthError::AccountConfirmationFailed)?;

    // Token is not used
    if token.used {
        return Err(AuthError::AccountConfirmationFailed(anyhow::anyhow!(
            "Admin token is already activated!"
        )));
    }

    let rows_count = user_auth_queries::use_admin_signup_token()
        .bind(client, &token.token)
        .await
        .context("Failed to set admin token's used to 'true'")?;

    // Just in case
    if rows_count != 1 {
        return Err(AuthError::UnexpectedError(anyhow::anyhow!(
            "When was updating admin token, {rows_count} rows was updated"
        )));
    }

    Ok(())
}
