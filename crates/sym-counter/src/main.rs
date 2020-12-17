use std::path::Path;
use common::{git::Git, github::Github};

type Result<T> = anyhow::Result<T>;

#[tokio::main]
async fn main() -> Result<()> {
    let gh_token = std::env::var("GITHUB_TOKEN")?;
    let github = Github::new(gh_token)?;

    let (files_count, sym_count) = Git.changed_files()?
        .iter()
        .filter(|file| {
            file.ends_with("md")
        })
        .map(count_for_file)
        .fold((0, 0), |(files, symbols), count| {
            (files + 1, symbols + count)
        });

    println!(
        "Files: {}, symbols: {}",
        files_count,
        sym_count
    );

    github.comment_pr(format!("Файлов: {}, символов: {}", files_count, sym_count)).await
}

fn count_for_file<P: AsRef<Path>>(file: P) -> usize {
    match std::fs::read_to_string(file) {
        Ok(content) => content.chars().count(),
        Err(e) => {
            log::info!("Read file to string error: {:?}", e);
            0
        },
    }
}
