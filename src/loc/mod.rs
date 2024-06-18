mod get_gitignore_paths;

use std::{
    fs::{self, DirEntry, File},
    io::Read,
    vec,
};

use clap::ArgMatches;
use gix_ignore::{glob::wildmatch::Mode, parse};

fn check_if_dir(
    file_or_dir: &DirEntry,
    extension: Option<&String>,
    ignored_paths: &Vec<String>,
) -> Vec<String> {
    let path = file_or_dir.path();
    let path_name = path.as_os_str().to_str().unwrap().to_string();

    let is_gitignored_path = ignored_paths.into_iter().find(|pattern| {
        let hi = parse(pattern.as_bytes());
        match hi.into_iter().find(|(line, _, _)| {
            let is_match = line.matches(path_name[2..].as_bytes().into(), Mode::IGNORE_CASE);
            is_match
        }) {
            Some(_) => true,
            None => false,
        }
    });
    if let Some(_) = is_gitignored_path {
        return vec![];
    }

    if path.is_file() {
        let file = vec![path_name];

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
            nested_files.extend(check_if_dir(&entry.unwrap(), extension, ignored_paths));
        }

        return nested_files;
    }
}

pub fn run(matches: &ArgMatches) {
    let extension = matches.get_one::<String>("extension");
    let ignore_gitignore_files = matches.get_one::<String>("gitignore").unwrap();
    let parsing_directory = matches.get_one::<String>("directory").unwrap();

    let ignored_paths = get_gitignore_paths::run(ignore_gitignore_files);
    // Read the current directory
    let current_dir = fs::read_dir(parsing_directory).unwrap();

    let mut files: Vec<String> = Vec::new();

    for entry in current_dir {
        let entry = entry.unwrap();

        files.extend(check_if_dir(&entry, extension, &ignored_paths));
    }

    let mut lines_of_code: u64 = 0;
    for (_index, file_name) in files.iter().enumerate() {
        let mut file_content = String::new();
        let mut file =
            File::open(file_name).expect(&format!("Was unable to open found file: {}", file_name));
        let _ = file.read_to_string(&mut file_content);


        let file_content: Vec<&str> = file_content.split("\n").collect();
        lines_of_code += file_content.len() as u64;
    }

    println!(
        "Directory {} has total of {} lines of code",
        parsing_directory, lines_of_code
    );
}
