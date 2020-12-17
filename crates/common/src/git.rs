use crate::{shell::Shell, Result};

pub struct Git;

impl Git {
    pub fn changed_files(&self) -> Result<Vec<String>> {
        Ok(
            Shell::run(r##"git show -m --name-only -1 --format=format:"##)?
                .lines()
                .map(|item| item.to_string())
                .collect(),
        )
    }
}
