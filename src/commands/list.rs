use clap::{Arg, Command};
use owo_colors::OwoColorize;

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
