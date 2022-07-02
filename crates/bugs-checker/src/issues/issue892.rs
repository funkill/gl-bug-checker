use once_cell::sync::OnceCell;
use regex::Regex;
use crate::{
    errors::{Bug, ErrorDescription},
    IssueChecker,
};


const ISSUE_ID: &str = "https://github.com/rust-lang-ru/book/issues/892";

pub(crate) struct Issue892;

static REGEX: OnceCell<Regex> = OnceCell::new();

impl IssueChecker for Issue892 {
    fn check(&self, _: &str, translation: &str) -> Option<crate::errors::Bug> {
        let re = REGEX.get_or_init(|| Regex::new(r#"<!--\s*игнор.*\s*-->"#).unwrap());

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
        ISSUE_ID
    }
}

#[cfg(test)]
mod test {
    use crate::{
        errors::{Bug, ErrorDescription},
        IssueChecker,
    };

    use super::{Issue892, ISSUE_ID};

    macro_rules! test {
        ($({ $name:ident, $translation:literal, $expected:expr }),+) => {
            $(
                #[test]
                fn $name() {
                    let issue = Issue892;
                    let actual = issue.check("", $translation);

                    assert_eq!($expected, actual);
                }
            )+
        };
    }

    test!(
        { check_valid, "<!-- ignore -->", None },
        { check_translated, "<!-- игнорировать -->", Some(Bug::new(ISSUE_ID, vec![ErrorDescription::SimpleString(String::from(
            "<!-- игнорировать -->"
        ))])) },
        { check_translated_variants, "<!-- игнорирование -->" , Some(Bug::new(ISSUE_ID, vec![ErrorDescription::SimpleString(String::from(
            "<!-- игнорирование -->"
        ))])) },
        { check_spaces, "<!--   игнорирование\t-->" , Some(Bug::new(ISSUE_ID, vec![ErrorDescription::SimpleString(String::from(
            "<!--   игнорирование\t-->"
        ))])) },
        { check_without_spaces, "<!--игнорирование-->" , Some(Bug::new(ISSUE_ID, vec![ErrorDescription::SimpleString(String::from(
            "<!--игнорирование-->"
        ))])) }
    );
}
