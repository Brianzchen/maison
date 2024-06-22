mod check_if_dir;
mod get_gitignore_paths;

use std::{
    fs::{self, File},
    io::Read,
};

use clap::ArgMatches;

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

        files.extend(check_if_dir::run(&entry, extension, &ignored_paths));
    }

    let mut lines_of_code: u64 = 0;
    for (_index, file_name) in files.iter().enumerate() {
        let mut file_content = String::new();
        let mut file =
            File::open(file_name).expect(&format!("Was unable to open found file: {}", file_name));

        match file.read_to_string(&mut file_content) {
            Ok(_) => {
                let file_content: Vec<&str> = file_content.split("\n").collect();
                lines_of_code += file_content.len() as u64;
            }
            Err(_) => {}
        };
    }

    println!(
        "Directory {} has total of {} lines of code across {} files",
        parsing_directory, lines_of_code, files.len()
    );
}
