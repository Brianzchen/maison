mod git_undo;

use crate::git_undo::git_undo;

/// Repo maintenance CLI to help you keep your house in order
fn main() {
    let cmd = clap::Command::new("maison")
        .bin_name("maison")
        .subcommand_required(true)
        .subcommand(
            clap::command!("loc")
            .arg(clap::arg!(--"count"))
        )
        .subcommand(clap::command!("git-undo"));

    let matches = cmd.get_matches();
    let matches = matches.subcommand();

    match matches {
        Some((feature, _matches)) => {
            match feature {
                "loc" => println!("you called loc!"),
                "git-undo" => git_undo(),
                _=> {
                    panic!("You must call a valid feature for maison to run upon, try `maison loc` as an example");
                }
            }
        },
        _ => {}
    }
}
