use crate::{errors::Bug, IssueChecker};

pub(crate) struct Issue90;

impl IssueChecker for Issue90 {
    fn check(&self, _: &str, translation: &str) -> Option<Bug> {
        let errors: Vec<String> = translation.lines()
            .filter(|line| {
                line.contains("<comment>")
            })
            .map(Into::into)
            .collect();

        if errors.is_empty() {
            Some(Bug::new(self.issue_id(), errors))
        } else {
            None
        }
    }

    fn issue_id(&self) -> &'static str {
        "90"
    }
}
