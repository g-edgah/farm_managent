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
        return Err(ServerFnError::new("Username must be 3–32 characters"));
    }
    if password.len() < 8 {
        return Err(ServerFnError::new("Password must be at least 8 characters"));
    }
}
