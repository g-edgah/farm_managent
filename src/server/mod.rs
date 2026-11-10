mod auth;

pub use auth::{register_user, user_login, user_logout};

#[cfg(feature = "server")]
pub use auth::spawn_session_cleanup;

#[cfg(feature = "server")]
use dioxus::prelude::*;

#[cfg(feature = "server")]
use dioxus::fullstack::Lazy;

#[cfg(feature = "server")]
pub static DATABASE: Lazy<sqlx::PgPool> = Lazy::new(|| async move {
    let url = std::env::var("DATABASE_URL").map_err(|e| ServerFnError::new(e.to_string()))?;

    let pool = sqlx::PgPool::connect(&url)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    // run migrations automatically on startup.
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    dioxus::Ok(pool)
});
