use std::process::Command;

pub fn run() {
    println!("Running git undo on last commit");

    Command::new("git")
        .arg("reset")
        .arg("--soft")
        .arg("HEAD~1")
        .spawn()
        .unwrap();
}
