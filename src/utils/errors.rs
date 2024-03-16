use thiserror::Error;
use warp::reject::{Reject, Rejection};

#[derive(Error, Debug)]
pub enum PhotoError {
    #[error("Error connecting to database")]
    Database(#[from] sqlx::Error),
    #[error("Error uploading photo: {0}")]
    Upload(String),
    #[error("Error analyzing photo")]
    Analysis,
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Other error: {0}")]
    Other(String),
}

// Implement the Reject trait for PhotoError
impl Reject for PhotoError {}
