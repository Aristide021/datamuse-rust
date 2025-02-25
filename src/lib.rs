mod client;
mod models;
mod utils;
mod error;
pub mod endpoints;

pub use client::DatamuseClient;
pub use models::*;
pub use error::DatamuseError;
