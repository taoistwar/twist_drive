use std::sync::OnceLock;

/**
 * 数据根目录
 */
pub static DATA_DIR: OnceLock<String> = OnceLock::new();
/**
 * Web服务端口
 */
pub static WEB_PORT: OnceLock<u32> = OnceLock::new();
/**
 * 运行模型：dev 开发模型，其它生产模型。
 *  开发模式使用 log4rs-dev.yaml
 *  生成模式使用 log4rs-prod.yaml (优先) 或 log4rs-rel.yaml
 */
pub static MODE: OnceLock<String> = OnceLock::new();
