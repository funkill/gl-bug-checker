#![feature(str_split_once)]

pub mod github_event;
pub mod git;
pub mod github;
pub mod logger;
pub mod repo;
pub mod shell;

pub type Result<T> = anyhow::Result<T>;
