use std::fs::OpenOptions;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_timestamp() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros()
}

pub fn log(message: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("hash.log")
        .unwrap();
    writeln!(file, "{}: {}", get_timestamp(), message).unwrap();
}
