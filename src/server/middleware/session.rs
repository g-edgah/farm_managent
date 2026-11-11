// #[cfg(feature = "server")]
// use super::models::User;

#[cfg(feature = "server")]
use crate::server::DATABASE;

#[cfg(feature = "server")]
use crate::server::models::User;

#[cfg(feature = "server")]
use dioxus::prelude::*;

#[cfg(feature = "server")]
pub async fn get_current_user() -> Option<User> {
    let token = dioxus_cookie::get_internal("session")?;

    sqlx::query_as::<_, User>(
        "SELECT u.id, u.username, u.type FROM users u
        JOIN sessions s ON s.user_id = u.id
        WHERE s.token = $1 AND s.expires_at > NOW()",
    )
    .bind(&token)
    .fetch_optional(&*DATABASE)
    .await
    .ok()?
}
