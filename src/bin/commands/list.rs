use clap::ArgMatches;
use clap::{Arg, Command};
use owo_colors::{OwoColorize, Style};
use scoop::core::installed_apps;
use scoop::util::prelude::*;
use scoop::util::errors::CliResult;
pub fn cli() -> Command {
    clap::Command::new("list").about("List installed apps").arg(
        Arg::new("query")
            .help(format!(
                "{}",
                "Lists all installed apps, or the apps matching the supplied query.".green()
            ))
            .required(false)
            .index(1), // 第一个位置参数
    )
}

pub fn exec(gctx: &mut GlobalContext, args: &ArgMatches) -> CliResult {
    let scoop_app_dir = gctx.scoop_dir().join("apps");
    let apps = installed_apps(&scoop_app_dir).unwrap();
    let name_len;
    if let Some(longest) = apps.iter().max_by_key(|s| s.len()) {
        name_len = longest.len();
    } else {
        name_len = 10;
    }

    println!("Installed apps, {}: \n", apps.len());
    let style = Style::new().green().underline();
    println!(
        "{:<name_len$} {:<10} {:<10} {:<10} {:<10}",
        "Name".style(style),
        "Version".style(style),
        "Source".style(style),
        "Updated".style(style),
        "Info".style(style),
        name_len = name_len
    );
    for app in apps {
        println!("{}", app);
    }
    if let Some(query) = args.get_one::<String>("query") {
        println!(":{query}")
    }
    Ok(())
}
