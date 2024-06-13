use std::{
    fs::{self, DirEntry},
    vec,
};

use clap::ArgMatches;

fn check_if_dir(
    file_or_dir: &DirEntry,
    extension: Option<&String>,
    ignore_gitignore_files: bool,
) -> Vec<String> {
    let path = file_or_dir.path();

    if path.is_file() {
        let file = vec![path.as_os_str().to_str().unwrap().to_string()];

        match extension {
            Some(ext) => {
                match path.extension() {
                    Some(file_ext) => {
                        if &file_ext.to_str().unwrap().to_string() == ext {
                            return file;
                        }
                    }
                    None => {
                        if ext == "." {
                            return file;
                        }
                    }
                }
                return vec![];
            }
            None => file,
        }
    } else {
        let nested_dirs = fs::read_dir(path).unwrap();

        let mut nested_files: Vec<String> = vec![];
        for entry in nested_dirs {
            nested_files.extend(check_if_dir(
                &entry.unwrap(),
                extension,
                ignore_gitignore_files,
            ));
        }

        return nested_files;
    }
}

pub fn run(matches: &ArgMatches) {
    let extension = matches.get_one::<String>("extension");
    let ignore_gitignore_files = matches.get_one::<String>("gitignore").unwrap();
    let ignore_gitignore_files = ignore_gitignore_files == "true";
    // Read the current directory
    let current_dir = fs::read_dir(".").unwrap();

    let mut files: Vec<String> = Vec::new();

    for entry in current_dir {
        let entry = entry.unwrap();

        files.extend(check_if_dir(&entry, extension, ignore_gitignore_files));
    }

    for (_index, file_name) in files.iter().enumerate() {
        println!("{file_name}");
    }
}
