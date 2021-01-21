use crate::{git::Git, github_event::GithubEvent};

pub struct TranslationProject<'s> {
    git: Git,
    event: GithubEvent,
    translation_dir: &'s str,
    original_dir: &'s str,
}

impl<'s> TranslationProject<'s> {
    pub fn new(
        translation_dir: &'s str,
        original_dir: &'s str,
    ) -> crate::Result<TranslationProject<'s>> {
        let git = Git::in_current_dir()?;
        let event = GithubEvent::default()?;

        Ok(TranslationProject {
            git,
            event,
            translation_dir,
            original_dir,
        })
    }

    pub fn changed_file_pairs(&self, ends_pattern: &str) -> crate::Result<Vec<(String, String)>> {
        let head = self.event.head_sha()?;
        let base = self.event.base_sha()?;

        Ok(self
            .git
            .updated_files(&base, &head)?
            .into_iter()
            .filter(|s| s.to_lowercase().ends_with(ends_pattern))
            .filter_map(|translation| {
                translation
                    .strip_prefix(&self.translation_dir)
                    .map(|path| self.original_dir.clone().to_owned() + path)
                    .map(|original| (original, translation))
            })
            .collect())
    }
}
