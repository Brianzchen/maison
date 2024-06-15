use clap::ArgMatches;
use std::process::Command;

pub fn run(matches: &ArgMatches) {
    println!("Running git undo on last commit");

    let commits = match matches.get_one::<i16>("commits") {
        Some(commits) => commits,
        None => &1,
    };

    Command::new("git")
        .arg("reset")
        .arg("--soft")
        .arg(format!("HEAD~{}", commits))
        .spawn()
        .unwrap();
}
