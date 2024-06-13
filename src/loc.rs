use std::fs;

// fn track_file(mut files: Vec<(&String)>, file_name: &String) {
//     // Read the file into a string
//     // count number of lines of code then store both that
//     // and
//     // let file_content = fs::read_to_string(path)?;
//     files.push((file_name));
// }

pub fn run() {
    // Read the current directory
    let current_dir = fs::read_dir(".").unwrap();

    let mut files: Vec<String> = Vec::new();

    for entry in current_dir {
        let entry = entry.unwrap();
        let path = entry.path();

        // Check if the entry is a file
        if path.is_file() {
            files.push(path.to_str().unwrap().to_string());
        } else {
          // Keep going and recursively find finds
          let nested_dir = fs::read_dir(entry.path()).unwrap();
          println!("{:?}", nested_dir);

          for entry in nested_dir {
            files.push(entry.unwrap().path().to_str().unwrap().to_string());
          }
        }
    }

    for (_index, file_name) in files.iter().enumerate() {
        println!("{file_name}");
    }
}
