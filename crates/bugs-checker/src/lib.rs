#![feature(format_args_nl, box_patterns, box_syntax)]

use errors::{Bug, Bugs};
use issues::{issue68::Issue68, issue82::Issue82, issue90::Issue90};

pub mod errors;
pub mod issues;

pub trait IssueChecker {
    fn check(&self, original: &str, translation: &str) -> Option<Bug>;
    fn issue_id(&self) -> &'static str;
}

pub struct Checker {
    checks: Vec<Box<dyn IssueChecker>>,
}

impl Checker {
    pub fn new(checks: Vec<Box<dyn IssueChecker>>) -> Self {
        Checker { checks }
    }

    pub fn default_checks() -> Self {
        let checks = vec![
            box Issue68 as Box<dyn IssueChecker>,
            box Issue82 as Box<dyn IssueChecker>,
            box Issue90 as Box<dyn IssueChecker>,
        ];

        Checker::new(checks)
    }

    pub fn check_file(&self, pair: &TranslaitionPair) -> Option<Bugs> {
        let bugs = self
            .checks
            .iter()
            .filter_map(|pass| pass.check(&pair.original, &pair.translation))
            .collect::<Vec<Bug>>();

        if bugs.is_empty() {
            None
        } else {
            Some(Bugs::new(pair.filename.to_owned(), bugs))
        }
    }
}

pub struct TranslaitionPair<'filename> {
    pub filename: &'filename str,
    pub original: String,
    pub translation: String,
}
