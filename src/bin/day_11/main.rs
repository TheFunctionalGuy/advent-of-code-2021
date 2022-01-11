use std::fmt;

use helper::{lines_from_file, Point};

fn main() {
	// --- Part One ---
	println!("--- Part One ---");
	// Read file and convert to 2d vector of octopuses
	let unparsed_input = lines_from_file("src/bin/day_11/day_11.txt");

	let mut octopus_grid: Vec<Vec<Octopus>> = unparsed_input.iter()
		.map(|l| l.chars()
			.filter_map(|n| n.to_digit(10))
			.map(|n| Octopus { energy_level: n as u8, has_flashed: false })
			.collect())
		.collect();

	let height = octopus_grid.len();
	let width = octopus_grid[0].len();

	// Ensure all numbers have been parsed correctly
	assert_eq!(10, height);
	assert_eq!(10, width);

	// Run 100 iterations and count flashes
	let mut flash_count: u32 = 0;

	for iteration in 0.. {
		println!("Iteration {:3}", iteration + 1);
		// 1. Step: Increase energy level
		for line in &mut octopus_grid {
			for octopus in line {
				octopus.energy_level += 1;
			}
		}

		let mut to_flash: Vec<Point> = Vec::new();

		// 2. Step: Flash and propagate flashes
		#[allow(clippy::needless_range_loop)]
		for y in 0..height {
			for x in 0..width {
				if octopus_grid[y][x].energy_level > 9 {
					to_flash.push((x, y).into());
				}
			}
		}

		while let Some(point) = to_flash.pop() {
			let adjacent_points = get_adjacent_points(point.x, point.y, width, height);

			for adjacent_point in adjacent_points {
				let octopus: &mut Octopus = &mut octopus_grid[adjacent_point.y][adjacent_point.x];
				octopus.energy_level += 1;

				if !octopus.has_flashed && octopus.energy_level > 9 {
					// Check if already in vector
					if !to_flash.contains(&adjacent_point) {
						to_flash.push(adjacent_point);
					}
				}
			}

			// Flashing finished
			octopus_grid[point.y][point.x].has_flashed = true;
		}

		// --- Part Two ---
		if octopus_grid.iter()
			.flatten()
			.all(|o| o.has_flashed) {
			println!("\n--- Part Two ---");
			println!("First iteration all octopuses flash: {}", iteration + 1);
			break;
		}

		// 3. Step: Reset energy level of flashed octopus
		for line in &mut octopus_grid {
			for octopus in line {
				if octopus.has_flashed {
					flash_count += 1;
					octopus.has_flashed = false;
					octopus.energy_level = 0;
				}
			}
		}

		if iteration == 99 {
			println!("Total number of flashes after {} iterations: {}", iteration + 1, flash_count);
		}
	}
}

struct Octopus {
	energy_level: u8,
	has_flashed: bool,
}

impl fmt::Display for Octopus {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.energy_level)
	}
}

// Generate all adjacent points to given point (including diagonal ones)
fn get_adjacent_points(x: usize, y: usize, width: usize, height: usize) -> Vec<Point> {
	if x == 0 {
		if y == 0 {
			vec![
				(x + 1, y   ).into(),   // Right
				(x + 1, y + 1).into(),  // Right-Down
				(x    , y + 1).into(),  // Down
			]
		} else if y == height - 1 {
			vec![
				(x    , y - 1).into(),  // Up
				(x + 1, y - 1).into(),  // Right-Up
				(x + 1, y    ).into(),  // Right
			]
		} else {
			vec![
				(x    , y - 1).into(),  // Up
				(x + 1, y - 1).into(),  // Right-Up
				(x + 1, y    ).into(),  // Right
				(x + 1, y + 1).into(),  // Right-Down
				(x    , y + 1).into(),  // Down
			]
		}
	} else if x == width - 1 {
		if y == 0 {
			vec![
				(x    , y + 1).into(),  // Down
				(x - 1, y + 1).into(),  // Left-Down
				(x - 1, y    ).into(),  // Left
			]
		} else if y == height - 1 {
			vec![
				(x    , y - 1).into(),  // Up
				(x - 1, y    ).into(),  // Left
				(x - 1, y - 1).into(),  // Left-Up
			]
		} else {
			vec![
				(x    , y - 1).into(),  // Up
				(x    , y + 1).into(),  // Down
				(x - 1, y + 1).into(),  // Left-Down
				(x - 1, y    ).into(),  // Left
				(x - 1, y - 1).into(),  // Left-Up
			]
		}
	} else if y == 0 {
		vec![
			(x + 1, y    ).into(),  // Right
			(x + 1, y + 1).into(),  // Right-Down
			(x    , y + 1).into(),  // Down
			(x - 1, y + 1).into(),  // Left-Down
			(x - 1, y    ).into(),  // Left
		]
	} else if y == height - 1 {
		vec![
			(x    , y - 1).into(),  // Up
			(x + 1, y - 1).into(),  // Right-Up
			(x + 1, y    ).into(),  // Right
			(x - 1, y    ).into(),  // Left
			(x - 1, y - 1).into(),  // Left-Up
		]
	} else {
		vec![
			(x    , y - 1).into(),  // Up
			(x + 1, y - 1).into(),  // Right-Up
			(x + 1, y    ).into(),  // Right
			(x + 1, y + 1).into(),  // Right-Down
			(x    , y + 1).into(),  // Down
			(x - 1, y + 1).into(),  // Left-Down
			(x - 1, y    ).into(),  // Left
			(x - 1, y - 1).into(),  // Left-Up
		]
	}
}
