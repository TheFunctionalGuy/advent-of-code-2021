use helper::lines_from_file;

fn main () {
	// --- Part One ---
	println!("--- Part One ---");
	// Read file and convert to vector u32 value
	let unparsed_input = lines_from_file("src/bin/day_06/day_06.txt");
	let initial_state: Vec<u32> = unparsed_input[0].split(',')
		.filter_map(|n| n.parse().ok())
		.collect();

	// Ensure all numbers have been parsed correctly
	assert_eq!(300, initial_state.len());

	let mut current_fish_timers = initial_state.clone();

	// Simulate fish population for 80 days
	for day in 0..80 {
		let mut new_fish: Vec<u32> = Vec::new();

		// Update every fish
		for timer in &mut current_fish_timers {
			if *timer == 0 {
				*timer = 6;
				// Create new fish with timer of 8
				new_fish.push(8);
			} else {
				*timer -= 1;
			}
		}

		// Add new fishes to group
		current_fish_timers.append(&mut new_fish);
		println!("Number of fishes after {:2} day(s): {}", day, current_fish_timers.len());
	}


	// --- Part Two ---
	println!("\n--- Part Two ---");
	let mut current_fish_timers: [u64; 9] = [0; 9];

	// Add fishes to initial bins
	for value in &initial_state {
		current_fish_timers[*value as usize] += 1;
	}

	// Simulate fish population for 256 days
	for day in 0..256 {
		let new_fish = current_fish_timers[0];

		// Update every fish bin
		for i in 0..8 {
			current_fish_timers[i] = current_fish_timers[i + 1];
		}

		// Add new fishes to group of 6 and 8
		current_fish_timers[6] += new_fish;
		current_fish_timers[8] = new_fish;
		println!("Number of fishes after {:2} day(s): {}", day, current_fish_timers.iter().sum::<u64>());
	}
}
