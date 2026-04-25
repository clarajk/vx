use clap::Subcommand;
use nix::unistd::Uid;

pub fn sudo(program: impl AsRef<str>) -> std::process::Command {
    if Uid::current().is_root() {
        return std::process::Command::new(program.as_ref());
    }

    let mut cmd = std::process::Command::new("sudo");
    cmd.arg(program.as_ref());

    cmd
}

#[derive(clap::Parser, Debug)]
#[clap(version, author, about)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(clap::Args, Debug)]
pub struct AddArgs {
    #[clap(required_unless_present = "fzf", num_args = 1..)]
    /// One or more packages to install.
    pub packages: Vec<String>,

    #[clap(short, long)]
    /// Force reinstallation.
    pub force: bool,

    #[clap(short, long)]
    /// Assume yes to confirmation prompts.
    pub yes: bool,

    #[clap(short, long)]
    /// Perform a dry run to show what would be installed.
    pub dry_run: bool,

    #[clap(short = 'F', long)]
    /// Use fzf (if available) to select package(s) to install.
    pub fzf: bool,
}

#[derive(clap::Args, Debug)]
pub struct UpgradeArgs {
    #[clap(short, long)]
    /// Perform a dry run to show what would be upgraded.
    pub dry_run: bool,

    #[clap(short, long)]
    /// Assume yes to confirmation prompts.
    pub yes: bool,
}

#[derive(clap::Args, Debug)]
pub struct UpdateArgs {
    #[clap(short, long)]
    /// Perform a dry run to show what would be updated.
    pub dry_run: bool,

    #[clap(short, long)]
    /// Assume yes to confirmation prompts.
    pub yes: bool,
}

#[derive(clap::Args, Debug)]
pub struct RemoveArgs {
    /// One or more packages to remove.
    #[clap(required = true)]
    pub packages: Vec<String>,

    #[clap(short, long)]
    /// Assume yes to confirmation prompts.
    pub yes: bool,

    #[clap(short, long)]
    /// Perform a dry run to show what would be removed.
    pub dry_run: bool,
}

#[derive(clap::Args, Debug)]
pub struct CleanArgs {
    #[clap(short, long)]
    /// Remove orphaned packages.
    pub orphans: bool,

    #[clap(short, long)]
    /// Clean the cache of outdated packages.
    pub cache: bool,

    #[clap(short, long)]
    /// Perform a dry run to show what would be removed.
    pub dry_run: bool,

    #[clap(short, long)]
    /// Assume yes to confirmation prompts.
    pub yes: bool,
}

#[derive(clap::Args, Debug)]
pub struct FindArgs {
    /// The query string to search for.
    #[clap(required_unless_present = "fzf")]
    pub query: Option<String>,

    #[clap(short, long)]
    /// Use fzf (if available) to interactively select package(s) from the search results.
    pub fzf: bool,
}

#[derive(clap::Args, Debug)]
pub struct PinArgs {
    #[clap(required = true)]
    /// One or more packages to pin.
    pub packages: Vec<String>,
}

#[derive(clap::Args, Debug)]
pub struct UnpinArgs {
    #[clap(required = true)]
    /// One or more packages to unpin.
    pub packages: Vec<String>,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[clap(alias = "s")]
    /// Sync the XBPS repositories.
    Sync,

    #[clap(alias = "a")]
    /// Install packages.
    Add(AddArgs),

    #[clap(alias = "ug")]
    /// Upgrade all packages to their latest versions using existing repo data.
    Upgrade(UpgradeArgs),

    #[clap(alias = "up")]
    /// Perform a sync and full system update.
    Update(UpdateArgs),

    #[clap(alias = "rm")]
    /// Remove package(s).
    Remove(RemoveArgs),

    #[clap(alias = "c")]
    /// Cleans orphaned packages and outdated packages in the cache.
    Clean(CleanArgs),

    #[clap(alias = "f")]
    /// Find a package using a query string.
    Find(FindArgs),

    /// Marks package(s) as manually installed so the clean command doesn't try to remove it.
    Pin(PinArgs),

    /// Unpin manually pinned package(s).
    Unpin(UnpinArgs),

    #[clap(alias = "ls")]
    /// List packages.
    List(ListArgs),

    #[clap(alias = "r", subcommand)]
    /// Manage repositories.
    Repo(RepoCommand),
}

#[derive(Subcommand, Debug)]
pub enum ListCommand {
    #[clap(alias = "a")]
    /// Lists all packages on the system.
    All,

    #[clap(alias = "m")]
    /// List manually installed packages.
    Manual,

    #[clap(alias = "o")]
    /// List orphaned packages.
    Orphans,
}

#[derive(clap::Args, Debug)]
pub struct ListArgs {
    #[clap(short, long)]
    /// Enable verbose messages.
    pub verbose: bool,

    /// Optional query string.
    pub query: Option<String>,

    #[clap(short, long)]
    pub fzf: bool,

    #[clap(subcommand)]
    /// The type of listing to perform. If not provided, lists all packages on the system.
    pub command: ListCommand,
}

#[derive(Subcommand, Debug)]
pub enum RepoCommand {
    #[clap(alias = "a")]
    Add(RepoAddArgs),
    #[clap(alias = "rm")]
    Remove(RepoAddArgs),
    #[clap(alias = "ls")]
    List(RepoListArgs),
    #[clap(alias = "on", alias = "e")]
    Enable(RepoAddArgs),
    #[clap(alias = "off", alias = "d")]
    Disable(RepoAddArgs),
}

#[derive(clap::Args, Debug)]
pub struct RepoAddArgs {
    /// The friendly name for the repo. Used for other repo actions and easy identification.
    pub name: String,

    /// The actual repository URL.
    pub url: String,

    #[clap(short, long)]
    /// Add the repository in a disabled state (default is enabled).
    pub disabled: bool,
}

#[derive(clap::Args, Debug)]
pub struct RepoActionArgs {
    /// The name of the repository to operate on.
    pub name: String,
}

#[derive(clap::Args, Debug)]
pub struct RepoListArgs {
    #[clap(short, long)]
    /// Enable verbose messages showing the repository names, URLs, and enabled/disabled status.
    pub verbose: bool,

    #[clap(long)]
    /// Don't show enabled repositories.
    pub no_enabled: bool,

    #[clap(long)]
    /// Don't show disabled repositories.
    pub no_disabled: bool,
}
