use crate::cli::{
    AddArgs, CleanArgs, FindArgs, ListArgs, PinArgs, RemoveArgs, UnpinArgs, UpdateArgs,
    UpgradeArgs, sudo,
};
use std::io::Result;
use std::process::{Command, ExitStatus, Stdio};
use which::which;

macro_rules! check {
    ($cond:expr) => {
        if !$cond {
            eprintln!("nothing to do");
            return Ok(ExitStatus::default());
        }
    };
}

fn no_fzf() -> ! {
    eprintln!("fzf was not found");
    eprintln!("hint: install it using `vx add fzf`");
    std::process::exit(1);
}

pub fn sync() -> Result<ExitStatus> {
    sudo("xbps-install").arg("--sync").status()
}

fn fzf_xbps_search(mut query: Command) -> Result<Vec<String>> {
    let mut query = query.stdout(Stdio::piped()).spawn()?;

    let Some(query_stdout) = query.stdout.take() else {
        return Err(std::io::Error::other("failed to capture xbps-query stdout"));
    };

    let fzf = Command::new("fzf")
        .stdin(Stdio::from(query_stdout))
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .arg("--multi")
        .arg("--header")
        .arg("TAB to select, ENTER to confirm")
        .arg("--bind")
        .arg("tab:toggle+down,shift-tab:toggle+up")
        .spawn()?;

    let output = fzf.wait_with_output()?;

    if !output.status.success() {
        return Ok(Vec::new());
    }

    let selected = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::to_owned)
        .collect();

    Ok(selected)
}

fn parse_xbps_line(line: &str) -> Option<(bool, String)> {
    let mut parts = line.split_whitespace();
    let status = parts.next()?; // "[*]" or "[-]" etc
    let pkgver = parts.next()?; // "foo-bar-1.2.3_1"
    let (name, _) = pkgver.rsplit_once('-')?;
    let installed = status.trim() == "[*]";

    Some((installed, name.to_string()))
}

pub fn add(args: AddArgs) -> Result<ExitStatus> {
    check!(!args.packages.is_empty() || args.fzf);

    let mut cmd = sudo("xbps-install");

    if args.force {
        cmd.arg("--force");
    }

    if args.dry_run {
        cmd.arg("--dry-run");
    }

    if args.yes {
        cmd.arg("--yes");
    }

    let selected = if args.fzf {
        if which("fzf").is_err() {
            no_fzf();
        }

        let mut query = Command::new("xbps-query");
        query.args(["-R", "--search", ""]);

        let pkgs = fzf_xbps_search(query)?;
        check!(!pkgs.is_empty());

        let mut filtered = Vec::with_capacity(pkgs.len());

        for line in pkgs {
            match parse_xbps_line(&line) {
                Some((true, name)) => {
                    eprintln!("{name} is already installed, skipping");
                }
                Some((false, name)) => {
                    filtered.push(name);
                }
                None => {
                    eprintln!("warning: failed to parse line: {line}");
                }
            }
        }

        filtered
    } else {
        Vec::new()
    };

    let mut all_pkgs = Vec::with_capacity(args.packages.len() + selected.len());
    all_pkgs.extend(args.packages);
    all_pkgs.extend(selected);

    check!(!all_pkgs.is_empty());
    cmd.args(all_pkgs).status()
}

pub fn upgrade(args: UpgradeArgs) -> Result<ExitStatus> {
    let mut cmd = sudo("xbps-install");
    cmd.arg("--update");

    if args.yes {
        cmd.arg("--yes");
    }

    if args.dry_run {
        cmd.arg("--dry-run");
    }

    cmd.status()
}

pub fn update(args: UpdateArgs) -> Result<ExitStatus> {
    let mut cmd = sudo("xbps-install");
    cmd.args(["--sync", "--update"]);

    if args.dry_run {
        cmd.arg("--dry-run");
    }

    if args.yes {
        cmd.arg("--yes");
    }

    cmd.status()
}

pub fn remove(args: RemoveArgs) -> Result<ExitStatus> {
    check!(!args.packages.is_empty());

    let mut cmd = sudo("xbps-remove");

    if args.yes {
        cmd.arg("--yes");
    }

    if args.dry_run {
        cmd.arg("--dry-run");
    }

    cmd.args(args.packages).status()
}

pub fn clean(args: CleanArgs) -> Result<ExitStatus> {
    check!(args.orphans || args.cache);

    let mut cmd = sudo("xbps-remove");

    if args.dry_run {
        cmd.arg("--dry-run");
    }

    if args.orphans {
        cmd.arg("--remove-orphans");
    }

    if args.cache {
        cmd.arg("--clean-cache");
    }

    if args.yes {
        cmd.arg("--yes");
    }

    cmd.status()
}

pub fn find(args: FindArgs) -> Result<ExitStatus> {
    if args.fzf {
        if which("fzf").is_err() {
            no_fzf();
        }

        let query_str = args.query.as_deref().unwrap_or("");
        let mut query = Command::new("xbps-query");
        query.args(["-R", "--search", query_str]);

        for line in fzf_xbps_search(query)? {
            println!("{line}");
        }

        Ok(ExitStatus::default())
    } else {
        Command::new("xbps-query")
            .arg("-R")
            .arg("--search")
            .arg(args.query.expect("clap requires query unless --fzf"))
            .status()
    }
}

fn set_mode(mode: impl AsRef<str>, pkgs: Vec<String>) -> Result<ExitStatus> {
    check!(!pkgs.is_empty());
    sudo("xbps-pkgdb")
        .arg("--mode")
        .arg(mode.as_ref())
        .args(pkgs)
        .status()
}

pub fn pin(args: PinArgs) -> Result<ExitStatus> {
    set_mode("manual", args.packages)
}

pub fn unpin(args: UnpinArgs) -> Result<ExitStatus> {
    set_mode("auto", args.packages)
}

pub fn list_all_pkgs(args: ListArgs) -> Result<ExitStatus> {
    let mut cmd = Command::new("xbps-query");

    if args.verbose {
        cmd.arg("--verbose");
    }

    if let Some(query) = args.query {
        cmd.args(["--search", &query]);
    } else {
        cmd.arg("--list-pkgs");
    }

    if args.fzf {
        for line in fzf_xbps_search(cmd)? {
            println!("{line}");
        }
    } else {
        cmd.status()?;
    }

    Ok(ExitStatus::default())
}

pub fn list_orphaned_pkgs(args: ListArgs) -> Result<ExitStatus> {
    let mut cmd = Command::new("xbps-query");
    cmd.arg("--list-orphans");

    if args.verbose {
        cmd.arg("--verbose");
    }

    if args.fzf {
        for line in fzf_xbps_search(cmd)? {
            println!("{line}");
        }
    } else {
        cmd.status()?;
    }

    Ok(ExitStatus::default())
}

pub fn list_manual_pkgs(args: ListArgs) -> Result<ExitStatus> {
    let mut cmd = Command::new("xbps-query");
    cmd.arg("--list-manual-pkgs");

    if args.verbose {
        cmd.arg("--verbose");
    }

    if args.fzf {
        for line in fzf_xbps_search(cmd)? {
            println!("{line}");
        }
    } else {
        cmd.status()?;
    }

    Ok(ExitStatus::default())
}
