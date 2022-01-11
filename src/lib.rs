use std::fmt;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::num::ParseIntError;
use std::path::Path;
use std::str::FromStr;


// === Loose helper functions ===
pub fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("No such file found.");
    let buffered_reader = BufReader::new(file);

    buffered_reader.lines()
        .map(|line| line.expect("Could not parse line"))
        .collect()
}


// === Data structures ===
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Point {
	pub x: usize,
	pub y: usize,
}

// Implementation to print Point
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Implementation to create Point from str
impl FromStr for Point {
	type Err = ParseIntError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let coords: Vec<_> = s.split(',').collect();

		let x = coords[0].parse()?;
		let y = coords[1].parse()?;

		Ok(Point { x, y })
	}
}

// Implementation to create Point from usize tuple
impl From<(usize, usize)> for Point {
	fn from((x, y): (usize, usize)) -> Self {
		Point { x, y }
	}
}
