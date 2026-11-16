pub mod claude;
pub mod codex;
pub mod config;
pub mod copy;
pub mod events;
pub mod ingest;
pub mod mask;
pub mod store;
pub mod t3;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
