use std::path::Path;

use crate::parse_args::{parse_args, Subcommand};
use anyhow::Result;
use inotify::{Inotify, WatchMask, EventMask};

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
    let mut inotify = Inotify::init()?;

    inotify.add_watch(
        file,
        WatchMask::MODIFY | WatchMask::DELETE_SELF
    )?;

    let mut buffer = [0u8; 4096];
    let file = file.to_string_lossy();

    'main: loop {
        let events = inotify
            .read_events_blocking(&mut buffer)?;
        
        for event in events {
            if event.mask.contains(EventMask::DELETE_SELF) {
                eprintln!("'{}' deleted, exiting...", file);
                break 'main Ok(())
            }
            if event.mask.contains(EventMask::MODIFY) {
                eprintln!("'{}' modified!", file);
            }
        }
    }
}
