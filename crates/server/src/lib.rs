mod arguments;
mod commons;
mod constants;
pub mod file;

use std::net::SocketAddr;

pub use constants::*;

use log::debug;

pub fn init() {
    // 解析命令行参数
    arguments::parse_args();
    // 配置日志
    commons::init_logs();
}
pub fn address() -> SocketAddr {
    let address = format!("0.0.0.0:{}", constants::WEB_PORT.get().unwrap());

    debug!("address: {}", address);
    match address.parse::<SocketAddr>() {
        Ok(v) => v,
        Err(_) => panic!("address fail:{}", address),
    }
}
