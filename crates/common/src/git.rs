use git2::{Oid, Repository};
use std::path::Path;

pub struct Git {
    repo: Repository,
}

impl Git {
    pub fn new<P: AsRef<Path>>(path: &P) -> crate::Result<Git> {
        Repository::open(path)
            .map(|repo| Git { repo })
            .map_err(Into::into)
    }

    pub fn in_current_dir() -> crate::Result<Git> {
        Git::new(&".")
    }

    pub fn updated_files(&self, from: &str, to: &str) -> crate::Result<Vec<String>> {
        let from = self.repo.find_commit(Oid::from_str(from)?)?.tree()?;
        let to = self.repo.find_commit(Oid::from_str(to)?)?.tree()?;

        Ok(self
            .repo
            .diff_tree_to_tree(Some(&from), Some(&to), Default::default())?
            .deltas()
            .filter(|delta| delta.new_file().exists())
            .flat_map(|delta| delta.new_file().path())
            .flat_map(|path| path.to_str())
            .map(String::from)
            .collect())
    }
}
