use std::sync::OnceLock;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "MiniStorage", about = "An example of StructOpt usage.")]
pub struct Opt {
    #[structopt(short = "a", long = "action", default_value = "download")]
    action: String,
    #[structopt(short = "s", long = "server")]
    server: String,
    #[structopt(short = "d", long = "data")]
    data_dir: String,
    #[structopt(short = "m", long = "mode", default_value = "dev")]
    mode: String,
}
pub static ARGS: OnceLock<Opt> = OnceLock::new();
fn main() {
    match Opt::from_args_safe() {
        Ok(args) => {
            ARGS.get_or_init(|| args);
        }
        Err(e) => {
            panic!("parse args fail:{}", e)
        }
    }
    println!("{:?}", ARGS.get());
}
