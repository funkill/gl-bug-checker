use crate::{git::Git, Result};

pub struct TranslationRepo<'s> {
    translation_dir: &'s str,
    original_dir: &'s str,
}

impl<'s> TranslationRepo<'s> {
    pub fn new(translation_dir: &'s str, original_dir: &'s str) -> TranslationRepo<'s> {
        TranslationRepo {
            translation_dir,
            original_dir,
        }
    }

    pub fn changed_file_pairs(&self, ends_pattern: &str) -> Result<Vec<(String, String)>> {
        Ok(Git
            .changed_files()?
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
