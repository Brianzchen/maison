use std::fs::{self, DirEntry};

fn check_if_dir(file_or_dir: &DirEntry) -> Vec<String> {
    let path = file_or_dir.path();

    if path.is_file() {
        return vec![path.as_os_str().to_str().unwrap().to_string()];
    } else {
        let nested_dirs = fs::read_dir(path).unwrap();

        let mut nested_files: Vec<String> = vec![];
        for entry in nested_dirs {
            nested_files.extend(check_if_dir(&entry.unwrap()));
        }

        return nested_files;
    }
}

pub fn run() {
    // Read the current directory
    let current_dir = fs::read_dir(".").unwrap();

    let mut files: Vec<String> = Vec::new();

    for entry in current_dir {
        let entry = entry.unwrap();

        files.extend(check_if_dir(&entry));
    }

    for (_index, file_name) in files.iter().enumerate() {
        println!("{file_name}");
    }
}
