use helper::lines_from_file;

fn main() {
	// --- Part One ---
	println!("--- Part One ---");
	// Read file and convert to vector of vectors of u32 values
	let unparsed_input = lines_from_file("src/bin/day_09/day_09.txt");

	let height_map: Vec<Vec<u32>> = unparsed_input.iter()
		.map(|l| l.chars()
			.filter_map(|n| n.to_digit(10))
			.collect())
		.collect();

	// Ensure all numbers have been parsed correctly
	assert_eq!(100, height_map.len());
	assert_eq!(100, height_map[0].len());

	// Find low points
	let width = height_map.len();
	let height = height_map[0].len();

	let mut low_points: Vec<u32> = Vec::new();

	for x in 0..width {
		for y in 0..height {
			let adjacent_points =
			if x == 0 {
				if y == 0 {
					//    Right,     Down
					vec![(x + 1, y), (x, y + 1)]
				} else if y == height - 1 {
					//    Right       Up
					vec![(x + 1, y), (x, y - 1)]
				} else {
					//    Right,      Up,         Down
					vec![(x + 1, y), (x, y - 1), (x, y + 1)]
				}
			} else if x == width - 1 {
				if y == 0 {
					//    Left,       Dowm
					vec![(x - 1, y), (x, y + 1)]
				} else if y == height - 1 {
					//    Left,       Up
					vec![(x - 1, y), (x, y - 1)]
				} else {
					//    Left,       Up,         Down
					vec![(x - 1, y), (x, y - 1), (x, y + 1)]
				}
			} else {
				if y == 0 {
					//    Left,       Right,      Down
					vec![(x - 1, y), (x + 1, y), (x, y + 1)]
				} else if y == height - 1 {
					//    Left,       Right,      Up
					vec![(x - 1, y), (x + 1, y), (x, y - 1)]
				} else {
					//    Left,       Right,      Up,         Down
					vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
				}
			};

			// Check condition on all adjacent points
			if adjacent_points.iter().all(|(x_2, y_2)| height_map[x][y] < height_map[*x_2][*y_2]) {
				low_points.push(height_map[x][y]);
			}
		}
	}

	// Get risk levels
	let risk_level: Vec<u32> = low_points.iter()
		.map(|n| n + 1)
		.collect();

	println!("Sum of risk levels: {}", risk_level.iter().sum::<u32>())
}
