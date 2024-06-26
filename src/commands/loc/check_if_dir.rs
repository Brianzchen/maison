use gix_ignore::{glob::wildmatch::Mode, parse};
use std::fs::{self, DirEntry};

pub fn run(
    file_or_dir: &DirEntry,
    extension: Option<&String>,
    ignored_paths: &Vec<String>,
) -> Vec<String> {
    let path = file_or_dir.path();
    let path_name = path.as_os_str().to_str().unwrap().to_string();

    let is_gitignored_path = ignored_paths.into_iter().find(|pattern| {
        let pattern = parse(pattern.as_bytes());
        match pattern.into_iter().find(|(line, _, _)| {
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
            nested_files.extend(run(&entry.unwrap(), extension, ignored_paths));
        }

        return nested_files;
    }
}

#[cfg(test)]
mod tests {
    use std::{env::temp_dir, fs::remove_dir_all};

    fn teardown() {
        let temp_dir = temp_dir();

        let _ = remove_dir_all(temp_dir.join("check_dir"));
    }

    #[test]
    fn returns_a_list_of_files_in_path() {
        let _ = temp_dir();
        // File::create_new(&temp_dir.join("check_dir").join("blah.js")).unwrap();
        // File::create_new(&temp_dir.join("check_dir").join("foo.rs")).unwrap();

        teardown();
    }

    #[test]
    fn able_to_ignore_paths_correctly() {
        teardown();
    }

    #[test]
    fn can_filter_on_file_extension() {
        teardown();
    }

    #[test]
    fn giving_extension_as_dot_returns_files_without_extension() {
        teardown();
    }
}
