use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

pub fn read_file(file_path: &str) -> io::Result<Vec<String>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    reader.lines().collect()
}
