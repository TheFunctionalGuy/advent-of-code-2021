use helper::lines_from_file;

fn main () {
	// --- Part One ---
	println!("--- Part One ---");
	// Read file and convert to vector u32 value
	let unparsed_input = lines_from_file("src/bin/day_6/day_6.txt");
	let initial_state: Vec<u32> = unparsed_input[0].split(",")
		.filter_map(|n| n.parse().ok())
		.collect();

	// Ensure all numbers have been parsed correctly
	assert_eq!(300, initial_state.len());

	let mut current_fish_timers = initial_state;

	// Simulate fish population for 80 days
	for day in 0..80 {
		let mut new_fish: Vec<u32> = Vec::new();

		// Update every fish
		for i in 0..current_fish_timers.len() {
			if current_fish_timers[i] == 0 {
				current_fish_timers[i] = 6;
				// Create new fish with timer of 8
				new_fish.push(8);
			} else {
				current_fish_timers[i] -= 1;
			}
		}

		// Add new fishes to group
		current_fish_timers.append(&mut new_fish);
		println!("Number of fishes after {:2} day(s): {}", day, current_fish_timers.iter().count());
	}
}
