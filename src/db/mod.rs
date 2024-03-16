pub mod models; // Assuming models.rs is in the same folder
pub mod repository; // Assuming repository.rs is in the same folder

pub use crate::db::models::Photo;  // Re-export Photo struct
pub mod database; // Assuming database.rs is in the same folder

pub use crate::db::database::connect;
// Optionally, re-export specific functions from repository.rs
// pub use crate::db::repository::insert;  // Example
