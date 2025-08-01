use clap::{Arg, Command};
use owo_colors::OwoColorize;

pub fn cli() -> Command {
    clap::Command::new("uninstall")
        .about("Uninstall an app")
        .arg(
            Arg::new("app")
                .help(format!("{}", "Remove file".green()))
                .required(true)
                .index(1), // 第一个位置参数
        )
}
