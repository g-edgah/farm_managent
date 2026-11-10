use dioxus::prelude::*;

use dioxus_cookie::{CookieOptions, SameSite};
use std::time::Duration;

#[cfg(feature = "server")]
use super::DATABASE;

#[cfg(feature = "server")]
use argon2::{Argon2, PasswordHasher};

// register
#[post("/api/auth/register")]
pub async fn register_user(
    username: String,
    password: String,
    confirm_password: String,
) -> Result<(), ServerFnError> {
    let username = username.trim().to_string();
    let password = password.trim().to_string();
    let confirm_password = confirm_password.trim().to_string();

    //input validation
    if username.len() < 3 || username.len() > 32 {
        return Err(ServerFnError::new(
            "Username must be at least 3 characters long",
        ));
    }
    if password.len() < 8 {
        return Err(ServerFnError::new(
            "Password must be at least 8 characters long",
        ));
    }
    if confirm_password != password {
        return Err(ServerFnError::new("Passwords do not match"));
    }

    //password hashing
    let hash = Argon2::default()
        .hash_password(password.as_bytes())
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .to_string();

    //database operation

    sqlx::query("INSERT INTO users (username, password_hash) VALUES ($1, $2)")
        .bind(&username)
        .bind(&hash)
        .execute(&*DATABASE)
        .await
        .map_err(|e| {
            if e.to_string().contains("duplicate key") {
                ServerFnError::new("Username unavailable");
                dioxus_logger::tracing::info!("db error during registration: {e}");
            } else {
                dioxus_logger::tracing::info!("db error during registration: {e}");
                ServerFnError::new("Could not create account");
            }
        })?;

    Ok(())
}

// login
#[post("/api/auth/login")]
pub async fn user_login(
    username: String,
    password: String,
    remember: bool,
) -> Result<(), ServerFnError> {
    let username = username.trim().to_string();
    let password = password.trim().to_string();

    // input validation
    if username.is_empty() || password.is_empty() {
        return Err(ServerFnError::new(
            "Username must be at least 3 characters long",
        ));
    }

    //password hashing
    let hash = Argon2::default()
        .hash_password(password.as_bytes())
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .to_string();

    //database operation
    let row: Option<(i32, String)> =
        sqlx::query_as("SELECT password_hash FROM users WHERE username = $1")
            .bind(&username)
            .fetch_optional(&DATABASE)
            .await
            .map_err(|e| {
                dioxus_logger::tracing::info!("db error during registration: {e}");
                ServerFnError::new("could not complete login")
            })?;

    let (user_id, stored_hash) = match row {
        Some(r) => r,
        None => return Err(ServerFnError::new("invalid username or password")),
    };

    let parsed_hash = argon2::PasswordHash::new(&stored_hash)
        .map_err(|_| ServerFnError::new("invalid username or password"))?;

    argon2::Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_err(|_| ServerFnError::new("invalid username or password"))?;

    //delete existing sessions before issuing a new one
    sqlx::query("DELETE FROM sessions WHERE user_id = $1")
        .bind(user_id)
        .execute(&DATABASE)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    //session isuing
    // generating cookie
    let token = uuid::Uuid::new_v4().to_string();
    let max_age_secs = 7 * 24 * 3600; // a week

    //storing new session
    sqlx::query(
        "INSERT INTO sessions (token, user_id, expires_at) VALUES ($1, $2, NOW() + ($3 || ' seconds')::interval)"
    )
        .bind(&token)
        .bind(user_id)
        .bind(max_age_secs)
        .execute(&DATABASE)
        .await
        .map_err(|e| ServerFnError::new("failed to save session")
    )?;

    dioxus_cookie::set(
        "session",
        &token,
        &CookieOptions {
            max_age: Some(Duration::from_secs(max_age_secs)),
            http_only: true,             // anti xss
            secure: true,                // https only. anti-MITM attacks
            same_site: SameSite::Strict, // anti CSRF
            path: "/".to_string(),
        },
    )?;

    Ok(())
}

// log out
#[post("/api/auth/logout")]
pub async fn user_logout() -> Result<(), ServerFnError> {
    let token = dioxus_cookie::get("session").ok_or_else(|| ServerFnError::new("No session"))?;

    sqlx::query("DELETE FROM sessions WHERE token =$1")
        .bind(&token)
        .execute(&*DATABASE)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    dioxus_cookie::clear("session")?;

    Ok(())
}

// periodic clearing of old sessions
#[cfg(feature = "server")]
pub fn spawn_session_cleanup() {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(3600)); // checks every hour
        loop {
            interval.tick().await;
            if let Ok(_) = sqlx::query("DELETE FROM sessions WHERE expires_at <NOW()")
                .execute(&*DATABASE)
                .await
            {
                dioxus_logger::tracing::info!("cleaned expired sessions");
            } else {
                dioxus_logger::tracing::info!("failed to clean up expired sessions");
            }
        }
    });
}
