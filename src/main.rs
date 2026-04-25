use crate::cli::{Args, Command, ListCommand, RepoCommand};
use crate::repo::Repositories;
use clap::Parser;
use std::process::ExitStatus;

mod cli;
mod repo;
mod xbps;

fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let mut repos = Repositories::open()?;

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
            RepoCommand::Add(args) => {
                repos.add(args.name, args.url, !args.disabled);
                repos.save()
            }
            RepoCommand::Remove(args) => {
                repos.remove(args.name);
                repos.save()
            }
            RepoCommand::List(args) => {
                let filtered: Vec<_> = repos
                    .iter()
                    .filter(|x| {
                        (x.enabled && !args.no_enabled) || (!x.enabled && !args.no_disabled)
                    })
                    .collect();

                let longest_name = filtered.iter().map(|x| x.name.len()).max().unwrap_or(0);

                for repo in filtered {
                    if args.verbose {
                        print!(
                            "{:10} ",
                            if repo.enabled {
                                "[enabled]"
                            } else {
                                "[disabled]"
                            }
                        );
                        print!("{:longest_name$} ", repo.name);
                        println!("({})", repo.url);
                    } else {
                        println!("{}", repo.url)
                    }
                }

                Ok(ExitStatus::default())
            }
            RepoCommand::Enable(args) => {
                repos.enable(args.name);
                repos.save()
            }
            RepoCommand::Disable(args) => {
                repos.disable(args.name);
                repos.save()
            }
        },
    }?;

    Ok(())
}
