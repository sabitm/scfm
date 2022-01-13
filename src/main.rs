use crate::parse_args::parse_args;
use anyhow::Result;

mod parse_args;

fn main() -> Result<()> {
    let _ = parse_args()?;

    println!("Hello, world!");

    Ok(())
}
