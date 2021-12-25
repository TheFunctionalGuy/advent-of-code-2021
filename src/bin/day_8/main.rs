use helper::lines_from_file;

fn main() {
	// --- Part One ---
	println!("--- Part One ---");
	// Read file and convert to vector u32 value
	let unparsed_input = lines_from_file("src/bin/day_8/day_8.txt");
	let signals: Vec<(Vec<_>, Vec<_>)> = unparsed_input.iter()
		.filter_map(|l| match l.split(" | ").collect::<Vec<_>>().as_slice() {
			[left, right] => {
				Some((
					left.split(" ").collect(),
					right.split(" ").collect(),
				))
			},
			_ => None,
		})
		.collect();

	// Ensure all numbers have been parsed correctly
	assert_eq!(200, signals.len());

	// Count number of easy digits
	let num_easy_patterns: usize = signals.iter()
		.map(|(_, right)| right.iter()
			.filter(|s| is_easy_pattern(s))
			.count())
		.sum();

	println!("Number of unique signal patterns: {}", num_easy_patterns);
}

// Easy digits use either 2 (=1), 3 (=7), 4 (=4) or 7 (=8) segments
fn is_easy_pattern(signal: &str) -> bool {
	let easy_segment_counts = [2, 3, 4, 7];

	easy_segment_counts.contains(&signal.len())
}
