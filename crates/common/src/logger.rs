use log::Level;
use std::io::Write;

pub struct Logger;

impl Logger {
    pub fn init() {
        pretty_env_logger::formatted_builder()
            .format(|buf, record| match record.level() {
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
            })
            .parse_filters(&std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()))
            .init();
    }
}
