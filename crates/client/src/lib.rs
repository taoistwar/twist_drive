mod client_commands;
mod client_commons;

pub use client_commands::*;
pub use client_commons::*;
use twist_drive_core::init_stdout_logs;

pub fn init_log(mode: &str) {
    let level = if mode.eq_ignore_ascii_case("dev") {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };
    init_stdout_logs(level);
}
