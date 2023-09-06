use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "MiniStorage", about = "An example of StructOpt usage.")]
pub struct Opt {
    #[structopt(short = "a", long = "action", default_value = "download")]
    pub action: String,
    #[structopt(short = "s", long = "server")]
    pub server: String,
    #[structopt(short = "l", long = "local-data-dir")]
    pub local_data_dir: String,
    #[structopt(short = "r", long = "remote-data-dir", default_value = "")]
    pub remote_data_dir: String,
    #[structopt(short = "m", long = "mode", default_value = "dev")]
    pub mode: String,

    #[structopt(
        short = "f",
        long = "force",
        default_value = "true",
        parse(try_from_str)
    )]
    pub force: bool,
}
