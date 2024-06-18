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
    #[test]
    fn returns_a_list_of_files_in_path() {}

    #[test]
    fn able_to_ignore_paths_correctly() {}

    #[test]
    fn can_filter_on_file_extension() {}

    #[test]
    fn giving_extension_as_dot_returns_files_without_extension() {}
}
