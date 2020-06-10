mod config;
mod git;
mod shell;

use anyhow::{anyhow, Result};
use bugs_checker::{
    errors::{Bugs, GLError},
    Checker, TranslaitionPair,
};
use clap::{App, Arg, ArgMatches};
use config::Config;
use git::TranslationRepo;

fn main() -> Result<()> {
    let config = configure()?;
    let checker = Checker::default_checks();
    let bugs = TranslationRepo::new(&config.translation_dir, &config.original_dir)
        .changed_file_pairs(".md")?
        .iter()
        .filter_map(|(original, translation)| {
            // todo: нужны нормальные ошибки
            let original_content = std::fs::read_to_string(&original).unwrap();
            let translation_content = std::fs::read_to_string(&translation).unwrap();
            let pair = TranslaitionPair {
                filename: translation,
                original: original_content,
                translation: translation_content,
            };

            checker.check_file(&pair)
        })
        .collect::<Vec<Bugs>>();

    if bugs.is_empty() {
        Ok(())
    } else {
        Err(GLError(bugs).into())
    }
}

fn configure() -> Result<Config> {
    fn get(matches: &ArgMatches, value: &str) -> Option<String> {
        matches.value_of(value).map(|dir| dir.to_owned())
    }

    let matches = App::new("checker")
        .args(&[
            Arg::with_name("original-dir")
                .long("original-dir")
                .short("o")
                .required(true)
                .takes_value(true),
            Arg::with_name("translation-dir")
                .long("translation-dir")
                .short("t")
                .required(true)
                .takes_value(true),
        ])
        .get_matches();

    let original_dir = get(&matches, "original-dir").ok_or_else(|| anyhow!("original-dir"))?;
    let translation_dir =
        get(&matches, "translation-dir").ok_or_else(|| anyhow!("translation-dir"))?;

    Ok(Config {
        original_dir,
        translation_dir,
    })
}
