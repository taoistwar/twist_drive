mod client_arguments;
mod client_constants;
mod client_errors;
pub use client_arguments::*;
pub use client_constants::*;
pub use client_errors::*;

use structopt::StructOpt;

pub async fn execute() -> anyhow::Result<()> {
    let args = Opt::from_args_safe()?;
    parse(&args).await
}

async fn parse(args: &Opt) -> anyhow::Result<()> {
    match &args.action as &str {
        "upload" | "u" => {
            crate::upload(args).await?;
        }
        "download" | "d" => {
            crate::download(args).await?;
        }
        _ => {
            print!("xx");
        }
    }
    Ok(())
}
