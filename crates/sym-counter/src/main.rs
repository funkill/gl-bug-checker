use common::{git::Git, github::Github, logger::Logger};
use std::path::Path;

type Result<T> = anyhow::Result<T>;

#[tokio::main]
async fn main() -> Result<()> {
    Logger::init();

    let users = std::env::var("INPUT_TAGGED_USERS").unwrap_or_default();

    let mut symbols = 0;
    let mut files = 0;
    let changed_files = Git.changed_files()?;
    for filename in changed_files {
        let symbols_count = count_for_file(&filename);
        if filename.ends_with(".md") {
            symbols += symbols_count;
            files += 1;
        }

        log::info!("File `{}`, symbols: {}", filename, symbols_count);
    }

    log::info!("Filtered files: {}, symbols: {}", files, symbols);

    let gh_token = std::env::var("GITHUB_TOKEN")?;
    Github::new(gh_token)?
        .comment_pr(format!(
            "Файлов: {}, символов: {}. {} fyi",
            files, symbols, users
        ))
        .await
}

fn count_for_file<P: AsRef<Path>>(file: P) -> usize {
    match std::fs::read_to_string(file) {
        Ok(content) => content.chars().count(),
        Err(e) => {
            log::warn!("Read file to string error: {:?}", e);
            0
        }
    }
}
