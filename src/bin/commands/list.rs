use std::fs;

use chrono::{DateTime, Local};
use clap::ArgMatches;
use clap::{Arg, Command};
use owo_colors::{OwoColorize, Style};
use scoop::core::installed_apps;
use scoop::core::manifest::{InstallInfo, Manifest};
use scoop::util::errors::CliResult;
use scoop::util::prelude::*;
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
    let local_apps = installed_apps(&scoop_app_dir, false).unwrap();
    let name_len;
    if let Some(longest) = local_apps.iter().max_by_key(|s| s.len()) {
        name_len = longest.len();
    } else {
        name_len = 10;
    }

    println!("Installed apps, {}: \n", local_apps.len());
    let style = Style::new().green().underline();
    println!(
        "{:<name_len$} {:<12} {:<10} {:<20} {:<10}",
        "Name".style(style),
        "Version".style(style),
        "Source".style(style),
        "Updated".style(style),
        "Info".style(style),
        // name_len = name_len
    );
    for app in local_apps {
        let mut app_dir = scoop_app_dir.join(&app);
        app_dir.push("current");
        let manifest_path = app_dir.join("manifest.json");
        let install_json_path = app_dir.join("install.json");
        // println!("{:?}", manifest_path);
        let data = fs::read_to_string(&manifest_path).expect("manifest.json read error!");
        let info = fs::read_to_string(&install_json_path).expect("install.json read error!");

        if let Ok(manifest) = serde_json::from_str::<Manifest>(&data) {
            if let Ok(install_info) = serde_json::from_str::<InstallInfo>(&info) {
                let metadata = fs::metadata(&install_json_path)?;
                let modified_time = metadata.modified()?; // SystemTime

                let datetime: DateTime<Local> = modified_time.into();

                println!(
                    "{:<name_len$} {:<12} {:<10} {:<20}",
                    app,
                    manifest.version,
                    install_info.bucket,
                    datetime.format("%Y-%m-%d %H:%M:%S")
                );
            }
        }
    }
    if let Some(query) = args.get_one::<String>("query") {
        println!(":{query}")
    }
    Ok(())
}
