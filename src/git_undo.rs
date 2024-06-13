use std::process::Command;
use clap::ArgMatches;

pub fn run(matches: &ArgMatches) {
    println!("Running git undo on last commit");

    let mut count = 1;
    match matches.get_one::<String>("count") {
        Some(value) => {
            let value = value.parse::<i32>();
            match value {
                Ok(value) => count = value,
                Err(_) => panic!("Value passed as count must be a whole number"),
            }
        },
        _ => {},
    };

    // Command::new("git")
    //     .arg("reset")
    //     .arg("--soft")
    //     .arg(format!("HEAD~{}", count))
    //     .spawn()
    //     .unwrap();
}
