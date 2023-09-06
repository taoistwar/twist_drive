use serde::{Deserialize, Serialize};
use structopt::StructOpt;

use crate::{DATA_DIR, MODE, WEB_PORT};

#[derive(Debug, StructOpt, Serialize, Deserialize)]
#[structopt(name = "MiniStorage", about = "An example of StructOpt usage.")]
pub struct Opt {
    #[structopt(short = "d", long = "data", default_value = "./data")]
    data_dir: String,
    #[structopt(short = "p", long = "port", default_value = "3000")]
    port: u32,

    #[structopt(short = "m", long = "mode", default_value = "dev")]
    mode: String,
}

impl Opt {}

pub fn parse_args() {
    match Opt::from_args_safe() {
        Ok(args) => {
            DATA_DIR.get_or_init(|| args.data_dir);
            WEB_PORT.get_or_init(|| args.port);
            MODE.get_or_init(|| args.mode);
        }
        Err(e) => {
            panic!("parse args fail:{}", e)
        }
    }
}
