//! The rok-dev command line, usable as the `rok-dev` binary or embedded in
//! the toolchain's `rok` (rok-v2) through [`command`] and [`run_from`].
//!
//! rok-dev runs a project's stack: it starts each **Runnable** named in
//! `rok.toml`'s `[dev.<name>]` tables, supervises it, and checks the
//! machine it runs on (rok-data's workspace ADR 0034). It owns four words:
//!
//! - `dev`: start the Runnables, in dependency order;
//! - `run`: one Runnable in the foreground, with its `.env`;
//! - `doctor`: check the toolchains and the known traps;
//! - `port`: find (`list`) and free (`free`) what holds a port.
//!
//! `rok` routes each of [`command`]'s subcommands here with the program
//! name `rok`, so help reads `rok dev …`; standalone it reads
//! `rok-dev dev …`.
//!
//! Exit codes: 0 for success and help, 1 for a runtime failure (a command
//! that is not built yet included), 2 for a usage error.

use clap::{Arg, ArgAction, Command};
use std::ffi::OsString;

/// Exit code for success.
pub const EXIT_OK: i32 = 0;
/// Exit code for a runtime failure.
pub const EXIT_FAILURE: i32 = 1;
/// Exit code for a usage error.
pub const EXIT_USAGE: i32 = 2;

/// The full clap command: one subcommand per word rok-dev owns.
pub fn command() -> Command {
    Command::new("rok-dev")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Run a project's stack: its Runnables, their devices and ports")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(dev())
        .subcommand(run())
        .subcommand(doctor())
        .subcommand(port())
}

fn dev() -> Command {
    Command::new("dev")
        .about("Start the project's Runnables from rok.toml's [dev] tables, in dependency order")
        .arg(
            Arg::new("names")
                .value_name("NAME")
                .action(ArgAction::Append)
                .help(
                    "Start only these Runnables (and what they depend on); all when none are named",
                ),
        )
}

fn run() -> Command {
    Command::new("run")
        .about("Run one Runnable in the foreground, in its dir, with its .env")
        .arg(
            Arg::new("name")
                .value_name("NAME")
                .required(true)
                .help("The Runnable to run"),
        )
}

fn doctor() -> Command {
    Command::new("doctor").about("Check the toolchains, devices and known traps, naming each fix")
}

fn port() -> Command {
    let ports = || {
        Arg::new("ports")
            .value_name("PORT")
            .action(ArgAction::Append)
            .value_parser(clap::value_parser!(u16))
    };
    Command::new("port")
        .about("Find and free what holds a port")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("list")
                .about("Show the process holding each [dev] port, or each given one")
                .arg(ports().help("The ports to look up; every [dev] port when none are given")),
        )
        .subcommand(
            Command::new("free")
                .about("Stop the process, and its tree, holding each given port")
                .arg(ports().required(true).help("The ports to free")),
        )
}

/// Parses `args` (the first item is the program name) and runs; returns the
/// exit code.
pub fn run_from<I, T>(args: I) -> i32
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let matches = match command().try_get_matches_from(args) {
        Ok(m) => m,
        Err(e) => {
            let _ = e.print();
            return e.exit_code();
        }
    };
    let ticket = match matches.subcommand_name() {
        Some("dev") => "`dev`: rok-dev tickets 02-04",
        Some("run") => "`run`: rok-dev ticket 05",
        Some("doctor") => "`doctor`: rok-dev ticket 08",
        Some("port") => "`port`: rok-dev ticket 09",
        _ => unreachable!("subcommand_required"),
    };
    eprintln!("error: not built yet ({ticket})");
    EXIT_FAILURE
}
