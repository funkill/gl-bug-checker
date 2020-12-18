use std::fmt::Display;

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
    file: String,
    bugs: Vec<Bug>,
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
pub struct Bug {
    issue_id: &'static str,
    errors: Vec<String>,
}

impl Bug {
    pub(crate) fn new(issue_id: &'static str, errors: Vec<String>) -> Self {
        Bug { issue_id, errors }
    }
}

impl Display for Bug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn format_errors(f: &mut std::fmt::Formatter<'_>, errors: &[String]) -> std::fmt::Result {
            for error in errors {
                f.write_fmt(format_args_nl!("\t{}", error))?;
            }

            Ok(())
        }

        f.write_fmt(format_args_nl!("\tIssue ID: #{}", self.issue_id))?;
        f.write_str("\tContent:\n")?;
        format_errors(f, &self.errors)
    }
}
