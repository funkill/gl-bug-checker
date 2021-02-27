use crate::{
    errors::{Bug, ErrorDescription},
    IssueChecker,
};

const ISSUE_ID: &str = "90";

pub(crate) struct Issue90;

impl IssueChecker for Issue90 {
    fn check(&self, _: &str, translation: &str) -> Option<Bug> {
        let errors: Vec<_> = translation
            .lines()
            .filter(|line| line.contains("<comment>"))
            .map(String::from)
            .map(ErrorDescription::SimpleString)
            .collect();

        if !errors.is_empty() {
            Some(Bug::new(self.issue_id(), errors))
        } else {
            None
        }
    }

    fn issue_id(&self) -> &'static str {
        ISSUE_ID
    }
}

#[cfg(test)]
mod tests {
    use super::{Issue90, ISSUE_ID};
    use crate::{errors::Bug, IssueChecker};

    macro_rules! test_gen {
        ($($name:ident, $translation:literal, $expected:expr),+) => {
            $(
                #[test]
                fn $name() {
                    let issue = Issue90;
                    let actual = issue.check("", $translation);

                    assert_eq!($expected, actual);
                }
            )+
        };
    }

    test_gen!(
        no_bug,
        "some text without comments",
        None,
        one_line,
        "<comment>some text",
        Some(Bug::new(ISSUE_ID, vec![String::from("<comment>some text")])),
        multiline,
        r#"some text
<comment>text after comment
some text
text before comment<comment>text after comment
some text"#,
        Some(Bug::new(
            ISSUE_ID,
            vec![
                String::from("<comment>text after comment"),
                String::from("text before comment<comment>text after comment")
            ]
        ))
    );
}
