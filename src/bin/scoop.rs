use clap::Command;
use scoop::util::prelude::*;
mod commands;
fn main() {
    let mut gctx = match GlobalContext::try_default() {
        Ok(gctx) => gctx,
        Err(_) => GlobalContext::default(),
    };
    let matches = Command::new("scoop")
        .version("1.0")
        .subcommands(commands::builtin())
        .get_matches();

    if let Some((cmd, sub_args)) = matches.subcommand() {
        let exec = commands::builtin_exec(cmd).unwrap();
        exec(&mut gctx, sub_args).unwrap();
    }

    // match matches.subcommand() {
    //     Some(("install", args)) => {
    //         // let file = sub_m.get_one::<String>("file").unwrap();
    //         // println!("Adding file: {}", file);
    //         if let Some(install) = args.get_one::<String>("install") {
    //             println!("Using install file: {}", install.green());
    //         }
    //     }
    //     Some(("uninstall", args)) => {
    //         // let file = sub_m.get_one::<String>("file").unwrap();
    //         // let force = sub_m.get_flag("force");
    //         // println!("Removing file: {}", file);
    //         // if force {
    //         //     println!("Force flag is set.");
    //         // }
    //         if let Some(uninstall) = args.get_one::<String>("uninstall") {
    //             println!("Using uninstall file: {}", uninstall.green());
    //         }
    //     }
    //     Some(("list", args)) => commands::list::exec(&mut gctx, args).unwrap(),
    //     _ => {
    //         println!("Use `add` or `remove` subcommand.");
    //     }
    // }
}
