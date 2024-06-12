use std::process::Command;

pub fn git_undo() {
    Command::new("sh")
        .arg("git reset --soft HEAD~1")
        .output()
        .unwrap();
}
