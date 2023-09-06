mod route;
mod server_commons;

use std::net::SocketAddr;

pub use route::*;
pub use server_commons::*;

use log::debug;

pub fn init() {
    // 解析命令行参数
    parse_args();
    // 配置日志
    init_logs();
}
pub fn address() -> SocketAddr {
    let address = format!("0.0.0.0:{}", WEB_PORT.get().unwrap());

    debug!("address: {}", address);
    match address.parse::<SocketAddr>() {
        Ok(v) => v,
        Err(_) => panic!("address fail:{}", address),
    }
}
