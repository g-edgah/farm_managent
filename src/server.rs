use dioxus::prelude::*;

#[server]
pub async fn register_user(
    username: String,
    password: String,
    confirm_password: String,
) -> Result<(), ServerError> {
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
    use argon2::password_hash::{rand_core::OsRng, SaltString};
    use argon2::{Argon2, PasswordHasher};

    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .to_string();

    //database operation
    let State(pool): State<sqlx::PgPool> = extract().await?;
    sqlx::query("INSERT INTO users (username, password_hash) VALUES ($1, $2)")
        .bind(&username)
        .bind(&hash)
        .execute(&pool)
        .await
        .map_err(|e| {
            if e.to_string().contains("duplicate key") {
                ServerFnError::new("Username unavailable")
            } else {
                ServerFnError::new("Could not create account")
            }
        })?;

    Ok(())
}
