use std::path::Path;
use std::path::PathBuf;

use argh::FromArgs;
use anyhow::{Result, bail};

#[derive(FromArgs)]
/// Simple Configuration Manager
struct Args {
    #[argh(subcommand)]
    subcmd: Subcommand,
}

#[derive(FromArgs)]
#[argh(subcommand)]
pub enum Subcommand {
    Watch(WatchSub),
}

#[derive(FromArgs)]
#[argh(subcommand, name = "watch")]
/// Watch for config changes and sync
pub struct WatchSub {
    #[argh(positional)]
    /// path to config file
    file: PathBuf,
}

impl WatchSub {
    pub fn get_file_path(&self) -> &Path {
        self.file.as_path()
    }
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

pub fn parse_args() -> Result<Subcommand> {
    let args: Args = argh::from_env();

    validate(&args)?;
    Ok(args.subcmd)
}
