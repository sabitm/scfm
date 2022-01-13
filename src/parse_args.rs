use std::path::PathBuf;

use argh::FromArgs;
use anyhow::{Result, bail};

#[derive(FromArgs)]
/// Simple Configuration Manager
pub struct Args {
    #[argh(subcommand)]
    subcmd: Subcommand,
}

#[derive(FromArgs)]
#[argh(subcommand)]
enum Subcommand {
    Watch(WatchSub),
}

#[derive(FromArgs)]
#[argh(subcommand, name = "watch")]
/// Watch for config changes and sync
struct WatchSub {
    #[argh(positional)]
    /// path to config file
    file: PathBuf,
}

fn validate(args: &Args) -> Result<()> {
    // check if file exist
    match &args.subcmd {
        Subcommand::Watch(wsub) => {
            let file = &wsub.file;
            if !file.is_file() {
                bail!("'{}' isn't a valid file!", file.to_string_lossy());
            }
        },
    }

    Ok(())
}

pub fn parse_args() -> Result<Args> {
    let args: Args = argh::from_env();

    validate(&args)?;
    Ok(args)
}
