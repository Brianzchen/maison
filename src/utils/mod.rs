use std::time::SystemTime;

pub fn log(enabled: bool, message: String) {
    if enabled {
        println!("{message}");
    }
}

pub fn end_time(enabled: bool, start_time: SystemTime) {
    let cur_time = SystemTime::now();

    let duration = cur_time.duration_since(start_time).unwrap();

    log(enabled, format!("Done in {}ms", duration.as_millis()))
}
