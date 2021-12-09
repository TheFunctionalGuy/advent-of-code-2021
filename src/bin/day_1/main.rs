use helper::lines_from_file;

fn main() {
	// --- Part One ---
	// Read file and convert to u32 vector
	let unparsed_input = lines_from_file("src/bin/day_1/day_1.txt");
	let depths: Vec<u32> = unparsed_input.iter()
		.filter_map(|line| line.parse::<u32>().ok())
		.collect();

	assert_eq!(depths.len(), 2000);

	// Count number of measurement increases
	let mut number_of_increases = 0;
	let mut previous_depth: Option<u32> = None;

	for depth in &depths {
		match previous_depth {
			Some(d) => {
				if d < *depth {
					number_of_increases += 1;
				}
				previous_depth = Some(*depth);
			},
			None => {
				previous_depth = Some(*depth);
			}
		}
	}

	println!("Number of measurement increases: {}", number_of_increases);


	// --- Part Two ---
	number_of_increases = 0;
	let mut previous_window_sum: Option<u32> = None;

	for i in 0..(depths.len() - 2) {
		let window_sum = depths[i] + depths[i+1] + depths[i+2];

		match previous_window_sum {
			Some(s) => {
				if s < window_sum {
					number_of_increases += 1;
				}
				previous_window_sum = Some(window_sum);
			}
			None => {
				previous_window_sum = Some(window_sum);
			}
		}
	}

	println!("Number of measurement increases using sliding windows: {}", number_of_increases);
}
