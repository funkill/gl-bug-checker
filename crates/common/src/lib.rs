#![feature(str_split_once)]

pub mod github_event;
pub mod git;
pub mod github;
pub mod logger;
pub mod shell;
pub mod translation_project;

pub type Result<T> = anyhow::Result<T>;
