pub mod install;
pub mod list;
pub mod uninstall;
use clap::Command;

pub fn builtin() -> Vec<Command> {
    vec![install::cli(), uninstall::cli(), list::cli()]
}
