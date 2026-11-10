mod navbar;
pub use navbar::Navbar;

// refers back to the mod.rs in /home
mod home;
pub use home::{Birds, Dashboard, Money};

// mod birds;
// pub use home::{};

// mod settings;
// pub use home::{};

mod profile;
pub use profile::{Details, Notifications, Privacy, Register};
