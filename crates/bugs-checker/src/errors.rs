use std::fmt::Display;

const GL_FEEDBACK_REPO: &str = "https://github.com/gitlocalize/feedback/issues/";

#[derive(Debug)]
pub struct GLError(pub Vec<Bugs>);

impl std::error::Error for GLError {}

impl Display for GLError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args_nl!("Found GitLocalize bugs:"))?;
        for err in &self.0 {
            f.write_fmt(format_args!("{}", err))?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Bugs {
    pub file: String,
    pub bugs: Vec<Bug>,
}

impl Bugs {
    pub(crate) fn new(file: String, bugs: Vec<Bug>) -> Self {
        Bugs { file, bugs }
    }
}

impl Display for Bugs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args_nl!("File: {}", self.file))?;
        for bug in &self.bugs {
            f.write_fmt(format_args!("{}", bug))?;
        }

        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ErrorDescription {
    SimpleString(String),
    Content {
        origin: String,
        translation: String,
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Bug {
    issue_id: &'static str,
    errors: Vec<ErrorDescription>,
}

impl Bug {
    pub(crate) fn new(issue_id: &'static str, errors: Vec<ErrorDescription>) -> Self {
        Bug { issue_id, errors }
    }
}

impl Display for Bug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn format_errors(f: &mut std::fmt::Formatter<'_>, errors: &[ErrorDescription]) -> std::fmt::Result {
            for error in errors {
                match error {
                    ErrorDescription::SimpleString(s) => {
                        f.write_fmt(format_args_nl!("\t{}", s))?;        
                    }
                    ErrorDescription::Content { origin, translation} => {
                        f.write_fmt(format_args_nl!("\tOrigin:{}\n\tTranslation{}", origin, translation))?;
                    }
                }
            }

            Ok(())
        }

        f.write_fmt(format_args_nl!(
            "\tLink to issue: {}{}",
            GL_FEEDBACK_REPO,
            self.issue_id
        ))?;
        f.write_str("\tContent:\n")?;
        format_errors(f, &self.errors)
    }
}
