use log::Level;
use std::{fmt::Display, io::Write};

const MAGICK_GITHUB_ACTION_TARGET: &str = "::github-action";

pub struct Logger;

impl Logger {
    pub fn init() -> Logger {
        pretty_env_logger::formatted_builder()
            .format(|buf, record| {
                if record.target() == MAGICK_GITHUB_ACTION_TARGET && record.level() == Level::Error
                {
                    return writeln!(buf, "{}", record.args());
                }

                match record.level() {
                    Level::Debug | Level::Trace => writeln!(buf, "::debug::{}", record.args()),
                    Level::Info | Level::Warn => writeln!(
                        buf,
                        "::warning file={},line={}::{}",
                        record.file().unwrap_or_default(),
                        record.line().unwrap_or_default(),
                        record.args()
                    ),
                    Level::Error => writeln!(
                        buf,
                        "::error file={},line={}::{}",
                        record.file().unwrap_or_default(),
                        record.line().unwrap_or_default(),
                        record.args()
                    ),
                }
            })
            .parse_filters(&std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()))
            .init();

        Logger
    }

    pub fn group<S: AsRef<str> + Display>(&self, name: S) {
        log::log!(
            target: MAGICK_GITHUB_ACTION_TARGET,
            Level::Error,
            "::group::{}",
            name
        );
    }

    pub fn end_group(&self) {
        log::log!(
            target: MAGICK_GITHUB_ACTION_TARGET,
            Level::Error,
            "::endgroup::"
        );
    }
}
