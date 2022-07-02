use once_cell::sync::OnceCell;
use regex::Regex;

use crate::{
    errors::{Bug, ErrorDescription},
    IssueChecker,
};

static REGEX: OnceCell<Regex> = OnceCell::new();

pub(crate) struct Issue68;

impl IssueChecker for Issue68 {
    fn check(&self, _: &str, translation: &str) -> Option<Bug> {
        let re = REGEX.get_or_init(|| Regex::new(r#"\{[\w]{1,6}[\d]{1,2}\}"#).unwrap());
        let errors: Vec<_> = re
            .captures_iter(translation)
            .map(|item| String::from(&item[0]))
            .map(ErrorDescription::SimpleString)
            .collect();

        if errors.is_empty() {
            None
        } else {
            Some(Bug::new(self.issue_link(), errors))
        }
    }

    fn issue_link(&self) -> &'static str {
        "https://github.com/gitlocalize/feedback/issues/68"
    }
}
