use std::process::Command;
use clap::ArgMatches;

pub fn run(matches: &ArgMatches) {
    println!("Running git undo on last commit");

    let count = match matches.get_one::<i16>("count") {
        Some(count) => count,
        None => &1,
    };

    Command::new("git")
        .arg("reset")
        .arg("--soft")
        .arg(format!("HEAD~{}", count))
        .spawn()
        .unwrap();
}
