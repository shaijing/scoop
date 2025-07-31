use clap::Command;
use owo_colors::OwoColorize;

use crate::core::scoop_list::list;

mod commands;
mod core;

fn main() -> std::io::Result<()> {
    let matches = Command::new("scoop")
        .version("1.0")
        .subcommands(commands::builtin())
        .get_matches();

    match matches.subcommand() {
        Some(("install", sub_m)) => {
            // let file = sub_m.get_one::<String>("file").unwrap();
            // println!("Adding file: {}", file);
            if let Some(install) = sub_m.get_one::<String>("install") {
                println!("Using install file: {}", install.green());
            }
        }
        Some(("uninstall", sub_m)) => {
            // let file = sub_m.get_one::<String>("file").unwrap();
            // let force = sub_m.get_flag("force");
            // println!("Removing file: {}", file);
            // if force {
            //     println!("Force flag is set.");
            // }
            if let Some(uninstall) = sub_m.get_one::<String>("uninstall") {
                println!("Using uninstall file: {}", uninstall.green());
            }
        }
        Some(("list", sub_m)) => list(sub_m)?,
        _ => {
            println!("Use `add` or `remove` subcommand.");
        }
    }
    Ok(())
}
