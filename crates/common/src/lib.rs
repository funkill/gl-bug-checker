pub mod git;
#[cfg(feature = "github-api")]
pub mod github;
pub mod github_event;
pub mod logger;
#[cfg(feature = "github-api")]
pub mod project;
pub mod shell;
pub mod translation_project;

pub type Result<T> = anyhow::Result<T>;
