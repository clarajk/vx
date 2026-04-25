use crate::cli::{Args, Command, ListCommand};
use clap::Parser;

mod cli;
mod xbps;

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    match args.command {
        Command::Sync => xbps::sync(),
        Command::Add(args) => xbps::add(args),
        Command::Upgrade(args) => xbps::upgrade(args),
        Command::Update(args) => xbps::update(args),
        Command::Remove(args) => xbps::remove(args),
        Command::Clean(args) => xbps::clean(args),
        Command::Find(args) => xbps::find(args),
        Command::Pin(args) => xbps::pin(args),
        Command::Unpin(args) => xbps::unpin(args),
        Command::List(args) => match args.command {
            ListCommand::All => xbps::list_all_pkgs(args),
            ListCommand::Orphans => xbps::list_orphaned_pkgs(args),
            ListCommand::Manual => xbps::list_manual_pkgs(args),
        },
    }?;

    Ok(())
}
