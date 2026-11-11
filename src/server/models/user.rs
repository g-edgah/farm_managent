#![cfg(feature = "server")]

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password_hash: String,
    pub r#type: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
