use std::path::Path;

use log::LevelFilter;
use log4rs::encode::json::JsonEncoder;
use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Root},
};
pub fn init_logs() {
    if crate::MODE.get().unwrap().eq_ignore_ascii_case("dev") {
        let file = "log4rs-dev.yaml";
        if Path::new(file).exists() {
            let msg = format!("{} init failed", file);
            log4rs::init_file(file, Default::default()).expect(&msg);
            return;
        }
        init_defined_logs(LevelFilter::Debug);
        return;
    }
    let file = "log4rs-prod.yaml";
    if Path::new(file).exists() {
        let msg = format!("{} init failed", file);
        log4rs::init_file(file, Default::default()).expect(&msg);
        return;
    }
    let file = "log4rs-rel.yaml";
    if Path::new(file).exists() {
        let msg = format!("{} init failed", file);
        log4rs::init_file(file, Default::default()).expect(&msg);
        return;
    }
    init_defined_logs(LevelFilter::Info);
}

fn init_defined_logs(level: LevelFilter) {
    let stdout: ConsoleAppender = ConsoleAppender::builder()
        .encoder(Box::new(JsonEncoder::new()))
        .build();

    let config = log4rs::config::Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(level))
        .unwrap();
    log4rs::init_config(config).expect("default log4rs init failed");
}
