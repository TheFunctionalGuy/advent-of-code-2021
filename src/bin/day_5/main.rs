use std::{str::FromStr, num::ParseIntError};

use helper::lines_from_file;

fn main() {
	// --- Part One ---
	println!("--- Part One ---");
	// Read file and convert to vector of tuple of points
	let unparsed_input = lines_from_file("src/bin/day_5/day_5.txt");
	let parsed_points: Vec<(Point, Point)> = unparsed_input.iter()
		.filter_map(|s| {
			let points: Vec<_> = s.split(" -> ").collect();
			Some((Point::from_str(points[0]).unwrap(), Point::from_str(points[1]).unwrap()))
		})
		.collect();

	// Enusre all points were parsed correctly
	assert_eq!(500, parsed_points.len());

	// Find maximum x and y value to determine diagram size
	let (max_x, max_y) = {
		let mut max_x = 0;
		let mut max_y = 0;

		for (from, to) in &parsed_points {
			if to.x > max_x {
				max_x = to.x;
			}
			if from.x > max_x {
				max_x = from.x;
			}
			if to.y > max_y {
				max_y = to.y;
			}
			if from.y > max_y {
				max_y = from.y;
			}
		}

		(max_x, max_y)
	};

	// Diagram to draw lines on
	let mut diagram: Vec<Vec<u32>> = vec![vec![0; max_x + 1]; max_y + 1];

	for (from, to) in &parsed_points {
		if from.x == to.x {
			// Swap coordinates if needed
			let (start, end) = if from.y < to.y { (from.y, to.y) } else { (to.y, from.y) };
			for i in start..=end {
				diagram[i][from.x] += 1;
			}
		} else if from.y == to.y {
			// Swap coordinates if neede
			let (start, end) = if from.x < to.x { (from.x, to.x) } else { (to.x, from.x) };
			for i in  start..=end {
				diagram[from.y][i] += 1
			}
		}
	}

	// Calculate number of overlaps
	let num_overlaps: usize = diagram.iter()
		.map(|l| l.iter()
			.filter(|&x| *x > 1).count()
		).sum();

	println!("Number of overlaps: {}", num_overlaps);
}

#[derive(Debug)]
struct Point {
	x: usize,
	y: usize,
}

// Implementation to create Point form str
impl FromStr for Point {
	type Err = ParseIntError;
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let coords: Vec<_> = s.split(",").collect();

		let x = coords[0].parse()?;
		let y = coords[1].parse()?;

		Ok(Point { x, y })
	}
}
