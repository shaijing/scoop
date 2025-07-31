use clap::{Arg, ArgAction, Command};

pub fn cli() -> Command {
    clap::Command::new("install")
        .about("Install apps")
        .arg(
            Arg::new("app").help("Install file").required(true).index(1), // 第一个位置参数
        )
        .arg(
            Arg::new("global")
                .short('g')
                .long("global")
                .action(ArgAction::SetTrue)
                .help("Install the app globally"),
        )
        .arg(
            Arg::new("independent")
                .short('i')
                .long("independent")
                .action(ArgAction::SetTrue)
                .help("Don't install dependencies automatically"),
        )
        .arg(
            Arg::new("no-cache")
                .short('k')
                .long("no-cache")
                .action(ArgAction::SetTrue)
                .help("Don't use the download cache"),
        )
        .arg(
            Arg::new("skip-hash-check")
                .short('s')
                .long("skip-hash-check")
                .action(ArgAction::SetTrue)
                .help("Skip hash validation (use with caution!)"),
        )
        .arg(
            Arg::new("no-update-scoop")
                .short('u')
                .long("no-update-scoop")
                .action(ArgAction::SetTrue)
                .help("Don't update Scoop before installing if it's outdated"),
        )
        .arg(
            Arg::new("arch")
                .short('a')
                .long("arch")
                .value_names(["32bit", "64bit", "arm64"])
                .help("Use the specified architecture, if the app supports it"),
        )
}
