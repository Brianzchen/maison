mod commands;
mod utils;

/// Repo maintenance CLI to help you keep your house in order
fn main() {
    let cmd = clap::Command::new("maison")
        .subcommand_required(true)
        .subcommand(clap::command!("loc")
            .arg(
                clap::Arg::new("extension")
                    .long("ext")
                    .short('c')
                    .help("Filter by a give file extension ie, rs or js")
                    .value_parser(clap::value_parser!(String)),
            )
            .arg(
                clap::Arg::new("gitignore")
                    .long("gitignore")
                    .help("Whether the command will respect the gitignore in the current working directory")
                    .value_parser(["true", "false"])
                    .default_value("true")
                    .ignore_case(true),
            )
            .arg(
                clap::Arg::new("directory")
                    .long("dir")
                    .short('d')
                    .help("Run command relative to current working directory")
                    .value_parser(clap::value_parser!(String))
                    .default_value("."),
            )
            .arg(
                clap::Arg::new("value")
                    .long("value")
                    .help("When you want to log only the loc value, useful when piping into another function later")
                    .value_parser(["true", "false"])
                    .default_value("false")
                    .ignore_case(true),
            )
            .arg(
                clap::Arg::new("timing")
                    .long("timing")
                    .short('t')
                    .help("Whether you want debug timings to print")
                    .value_parser(["true", "false"])
                    .default_value("false")
                    .ignore_case(true),
            ),
        )
        .subcommand(clap::command!("git-undo")
            .arg(
                clap::Arg::new("commits")
                    .long("commits")
                    .short('c')
                    .help("The number of commits to be undone")
                    .value_parser(clap::value_parser!(i16).range(1..10))
                    .default_value("1"),
            ),
        )
        .subcommand(clap::command!("ratchet")
            .arg(
                clap::Arg::new("name")
                    .long("name")
                    .short('n')
                    .help("Name of the ratchet to receive and reference later")
                    .value_parser(clap::value_parser!(String))
                    .required(true)
            ),
        );

    let matches = cmd.get_matches();
    let matches = matches.subcommand();

    match matches {
        Some((feature, feature_matches)) => match feature {
            "loc" => commands::loc::run(feature_matches),
            "git-undo" => commands::git_undo::run(feature_matches),
            "ratchet" => commands::ratchet::run(feature_matches),
            _ => {
                panic!("You must call a valid feature for maison to run upon, try `maison loc` as an example");
            }
        },
        _ => {}
    }
}
