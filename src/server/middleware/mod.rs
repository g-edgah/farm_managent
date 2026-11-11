mod session;

#[cfg(feature = "server")]
pub use session::get_current_user;
