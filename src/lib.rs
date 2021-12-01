use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;

pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("No such file found.");
    let buffered_reader = BufReader::new(file);

    buffered_reader.lines()
        .map(|line| line.expect("Could not parse line"))
        .collect()
}
