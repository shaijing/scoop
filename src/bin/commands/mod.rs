pub mod install;
pub mod list;
pub mod uninstall;
use clap::ArgMatches;
use clap::Command;
use scoop::util::errors::CliResult;
use scoop::util::prelude::GlobalContext;

pub fn builtin() -> Vec<Command> {
    vec![install::cli(), uninstall::cli(), list::cli()]
}
pub type Exec = fn(&mut GlobalContext, &ArgMatches) -> CliResult;

pub fn builtin_exec(cmd: &str) -> Option<Exec> {
    let f = match cmd {
        "list" => list::exec,
        _ => return None,
    };
    Some(f)
}
