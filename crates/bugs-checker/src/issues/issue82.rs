use crate::{
    errors::{Bug, ErrorDescription},
    IssueChecker,
};
use pulldown_cmark::{CowStr, Event, Parser, Tag};

pub(crate) struct Issue82;

fn get_code_blocks<'parser>(parser: &mut Parser<'parser>) -> Vec<CowStr<'parser>> {
    let mut codes = vec![];
    // let iter = parser.into_iter();
    while let Some(item) = parser.next() {
        if let Event::Start(Tag::CodeBlock(_)) = item {
            if let Some(Event::Text(code)) = parser.next() {
                codes.push(code);
            }
        }
    }

    codes
}

impl IssueChecker for Issue82 {
    fn check(&self, original: &str, translation: &str) -> Option<Bug> {
        let translation_code_blocks = {
            let mut markdown = pulldown_cmark::Parser::new(translation);
            get_code_blocks(&mut markdown)
        };

        if translation_code_blocks.is_empty() {
            return None;
        }

        let original_code_blocks = {
            let mut markdown = pulldown_cmark::Parser::new(original);
            get_code_blocks(&mut markdown)
        };

        if original_code_blocks.len() != translation_code_blocks.len() {
            panic!(
                "Code blocks has different length: translation is {} items and original is {} items\n{:?}",
                translation_code_blocks.len(),
                original_code_blocks.len(),
                translation_code_blocks
            );
        }

        let mut original_blocks_iter = original_code_blocks.iter();
        let mut translation_blocks_iter = translation_code_blocks.iter();
        let mut errors = vec![];
        while let (Some(translation_code), Some(original_code)) =
            (translation_blocks_iter.next(), original_blocks_iter.next())
        {
            if original_code.lines().count() < 2 {
                continue;
            }

            if translation_code.lines().count() == 1 {
                let desc = ErrorDescription::Content {
                    origin: original_code.to_string(),
                    translation: translation_code.to_string(),
                };
                errors.push(desc);
            }
        }

        if errors.is_empty() {
            None
        } else {
            Some(Bug::new(self.issue_id(), errors))
        }
    }

    fn issue_id(&self) -> &'static str {
        "82"
    }
}
