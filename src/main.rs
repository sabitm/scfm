use std::path::Path;

use crate::parse_args::{parse_args, Subcommand};
use anyhow::Result;

mod parse_args;

fn main() -> Result<()> {
    let cmd = parse_args()?;

    match cmd {
        Subcommand::Watch(wsub) => {
            start_watch(wsub.get_file_path())?;
        }
    }

    println!("Hello, world!");

    Ok(())
}

fn start_watch(file: &Path) -> Result<()> {
    

    Ok(())
}
