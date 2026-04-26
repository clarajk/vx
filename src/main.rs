use crate::cli::{refuse_root, Args, Command, ListCommand, RepoCommand};
use clap::Parser;

mod cli;
mod repo;
mod xbps;

fn main() -> std::io::Result<()> {
    refuse_root();

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
        Command::Repo(command) => match command {
            RepoCommand::Add(args) => xbps::add_repo(args),
            RepoCommand::Remove(args) => xbps::remove_repo(args),
            RepoCommand::List(args) => xbps::list_repos(args),
            RepoCommand::Enable(args) => xbps::enable_repo(args),
            RepoCommand::Disable(args) => xbps::disable_repo(args),
        },
    }?;

    Ok(())
}
