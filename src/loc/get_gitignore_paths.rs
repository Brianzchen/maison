use std::fs::File;
use std::io::Read;

pub fn run(gitignore: &String) -> Vec<String> {
    let ignore_gitignore_files = gitignore == "true";

    if !ignore_gitignore_files {
        return vec![];
    }

    let git_dir = String::from(".git/");

    // TODO: This should be traversed upward in a future task
    match File::open(".gitignore") {
        Ok(mut file) => {
            // let mut file = .expect("Found .gitignore file but failed to read");
            let mut contents = String::new();
            let _ = file.read_to_string(&mut contents);

            let lines: Vec<String> = contents.split('\n').map(|s| s.to_string()).collect();
            let mut lines: Vec<String> = lines
                .into_iter()
                .filter(|line| !line.starts_with("#") && !line.trim().is_empty())
                .collect();

            lines.push(git_dir);

            lines
        }
        Err(_) => {
            vec![git_dir]
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{env, fs, io::Write, path::PathBuf};

    use super::*;

    fn setup() -> PathBuf {
        let temp_dir = env::temp_dir();
        env::set_current_dir(&temp_dir).unwrap();

        let _ = fs::remove_file(&temp_dir.join(".gitignore"));

        return temp_dir;
    }

    #[test]
    fn returns_empty_vector_when_no_git_ignore() {
        setup();

        let gitignore = String::from("anything");
        let paths = run(&gitignore);

        assert!(paths.len() == 0);
    }

    #[test]
    fn adds_git_dir_by_default_when_no_gitignore_found() {
        setup();

        let gitignore = String::from("true");
        let paths = run(&gitignore);

        assert!(paths.contains(&String::from(".git/")));
        assert!(paths.len() == 1);
    }

    #[test]
    fn adds_git_dir_by_default() {
        let temp_dir = setup();

        let mut gitignore_file = File::create_new(&temp_dir.join(".gitignore")).unwrap();
        gitignore_file.write_all("src/\nblah**".as_bytes()).unwrap();

        let gitignore = String::from("true");
        let paths = run(&gitignore);

        assert!(paths.contains(&String::from(".git/")));
        assert!(paths.len() == 3);
    }
}
