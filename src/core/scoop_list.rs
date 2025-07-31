use std::{env, path::PathBuf};

use clap::ArgMatches;
use owo_colors::{OwoColorize, Style};

use crate::core::scoop_core::installed_apps;

pub fn list(arg: &ArgMatches) -> std::io::Result<()> {
    let scoop = env::var("SCOOP").unwrap_or("~/.scoop".to_string());
    let mut scoop_dir = PathBuf::from(scoop);
    scoop_dir.push("apps");
    let apps = installed_apps(&scoop_dir)?;
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
    if let Some(query) = arg.get_one::<String>("query") {
        println!(":{query}")
    }
    Ok(())
}
