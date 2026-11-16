pub mod config;
pub mod copy;
pub mod t3;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
