use std::fs;

pub fn run() {
    // Read the current directory
    let current_dir = fs::read_dir(".").unwrap();

    for entry in current_dir {
        let entry = entry.unwrap();
        let path = entry.path();

        // Check if the entry is a file
        if path.is_file() {
            // Read the file into a string
            // let file_content = fs::read_to_string(path)?;

            // // Print the file name and content
            // println!("File: {}", copied_path.display());
            println!("{}", path.display());
        } else {
          // Keep going and recursively find finds
          let nested_dir = fs::read_dir(entry.path()).unwrap();
          println!("{:?}", nested_dir);

          for _entry in nested_dir {

          }
        }
    }
}
