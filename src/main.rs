mod git_undo;
mod loc;

/// Repo maintenance CLI to help you keep your house in order
fn main() {
    let cmd = clap::Command::new("maison")
        .subcommand_required(true)
        .subcommand(
            clap::command!("loc")
        )
        .subcommand(
            clap::command!("git-undo")
            .arg(
                clap::Arg::new("count")
                    .long("commits")
                    .short('c')
                    .help("The number of commits to be undone")
                    .value_parser(clap::value_parser!(i16).range(1..10))
                    .default_value("1")
            ),
        );

    let matches = cmd.get_matches();
    let matches = matches.subcommand();

    match matches {
        Some((feature, feature_matches)) => {
            match feature {
                "loc" => loc::run(),
                "git-undo" => git_undo::run(feature_matches),
                _=> {
                    panic!("You must call a valid feature for maison to run upon, try `maison loc` as an example");
                }
            }
        },
        _ => {}
    }
}
