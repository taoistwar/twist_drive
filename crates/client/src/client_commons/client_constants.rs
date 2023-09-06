use std::sync::OnceLock;

use crate::Opt;

pub static ARGS: OnceLock<Opt> = OnceLock::new();
