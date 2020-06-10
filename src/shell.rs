use anyhow::Result;
use std::process::{Command, Stdio};

pub struct Shell;

impl Shell {
    pub fn run<C>(command: C) -> Result<String>
    where
        C: AsRef<str>,
    {
        let mut args: Vec<String> = command
            .as_ref()
            .split_whitespace()
            .map(ToString::to_string)
            .collect();
        let exec = args.remove(0);
        let mut command = Command::new(exec);
        let output = command
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .and_then(|child| child.wait_with_output())?;

        let stdout = String::from_utf8(output.stdout)?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);

            panic!(
                "Error: {}\nstderr: {}\nstdout: {}",
                output.status, stderr, stdout
            );
        }

        Ok(stdout)
    }
}
