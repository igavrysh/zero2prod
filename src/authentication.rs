use anyhow::Context;
use argon2::{PasswordHash, Argon2, PasswordVerifier};
use secrecy::{Secret, ExposeSecret};
use sqlx::PgPool;

use crate::telemetry::spawn_blocking_with_tracing;


#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials.")]
    InvalidCredentials(#[source] anyhow::Error),

    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error)
}

pub struct Credentials {
    pub username: String,
    pub password: Secret<String>,
}

#[tracing::instrument(name = "Validate credentials", skip(credentials, pool))]
pub async fn validate_credentials(
    credentials: Credentials,
    pool: &PgPool,
) -> Result<uuid::Uuid, AuthError> {
    let mut user_id = None;
    let mut expected_password_hash = Secret::new(
        "$argon2id$v=19$m=15000,t=2,p=1$\
        gZiV/M1gPc22ElAH/Jh1Hw$\
        CWOrkoo7oJBQ/iyh7uJ0LO2aLEfrHwTWllSAxT0zRno"
        .to_string()
    );

    if let Some((stored_user_id, stored_password_hash))
        = get_stored_credentials(
            &credentials.username,
            &pool
        )
        .await
        .map_err(AuthError::UnexpectedError)?
    {
        user_id = Some(stored_user_id);
        expected_password_hash = stored_password_hash;
    }

    spawn_blocking_with_tracing(move || {
        verify_password_hash(
            expected_password_hash, 
            credentials.password
        )
    })
    .await
    .context("Failed to spawn blocking task.")
    .map_err(AuthError::UnexpectedError)?
    .await?;

    user_id.ok_or_else(||
        AuthError::InvalidCredentials(anyhow::anyhow!("Unknown username."))
    )
}

#[tracing::instrument(name = "Get stored credentials", skip(username, pool))]
async fn get_stored_credentials(
    username: &str,
    pool: &PgPool
) -> Result<Option<(uuid::Uuid, Secret<String>)>, anyhow::Error> {
    let row
        = sqlx::query!(
            r#"
            SELECT user_id, password_hash
            FROM users
            WHERE username = $1
            "#,
            username,
        )
        .fetch_optional(pool)
        .await
        .context("Failed to perform a query to retireve stored credentials.")?
        .map(|r| (r.user_id, Secret::new(r.password_hash)));
    Ok(row)
}

#[tracing::instrument(
    name = "Verify password hash", 
    skip(expected_password_hash, password_candidate)
)]
async fn verify_password_hash(
    expected_password_hash: Secret<String>,
    password_candidate: Secret<String>
) -> Result<(), AuthError> {

    // TODO(gavrysh): why "?"" works in the end of statement? how it decides which type to use?
    let expected_password_hash 
        = PasswordHash::new(
            expected_password_hash.expose_secret()
        )
        .context("Failed to parse hash in PHC string format.")?;

    // vs this line? why below we should use map err?
    Argon2::default()
        .verify_password(
            password_candidate.expose_secret().as_bytes(), 
            &expected_password_hash,
        )
        .context("Invalid password.")
        .map_err(AuthError::InvalidCredentials)
}