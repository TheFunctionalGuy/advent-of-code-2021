use std::collections::{HashMap, HashSet};

use helper::lines_from_file;

fn main() {
	// --- Part One ---
	println!("--- Part One ---");
	// Read file and convert to vector of String tuples
	let unparsed_input = lines_from_file("src/bin/day_08/day_08.txt");
	let signals: Vec<(Vec<String>, Vec<String>)> = unparsed_input.iter()
		.filter_map(|l| match l.split(" | ").collect::<Vec<_>>().as_slice() {
			[left, right] => {
				Some((
					left.split(' ')
						// Normalize left string by sorting its characters
						.map(|s| {
							let mut chars = s.chars().collect::<Vec<char>>();
							chars.sort_unstable();
							String::from_iter(chars)
						}).collect(),
					right.split(' ')
						// Normalize right string by sorting its charactersq
						.map(|s| {
							let mut chars = s.chars().collect::<Vec<char>>();
							chars.sort_unstable();
							String::from_iter(chars)
						})
						.collect(),
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


	// --- Part Two ---
	println!("\n--- Part Two ---");

	// Generate vector of output values
	let output_values: Vec<u32> = signals.iter()
		.map(|(left, right)| apply_mapping(right, find_mapping(left)))
		.map(|n| n.iter()
			.fold(0, |acc, x| (acc * 10) + x))
		.collect();

	println!("Sum of output values: {}", output_values.iter().sum::<u32>())
}

// Easy digits use either 2 (=1), 3 (=7), 4 (=4) or 7 (=8) segments
fn is_easy_pattern(signal: &str) -> bool {
	let easy_segment_counts = [2, 3, 4, 7];

	easy_segment_counts.contains(&signal.len())
}

fn find_mapping(patterns: &[String]) -> HashMap<&String, u32> {
	// HashMap for the deduced mappings
	let mut mapping: HashMap<&String, u32> = HashMap::new();
	// HashMap from String to containing chars
	let mut set_mappings: HashMap<&String, HashSet<char>> = HashMap::new();

	let mut str_len_5: Vec<&String> = Vec::new();
	let mut str_len_6: Vec<&String> = Vec::new();

	// Only two HashSets needed for identification are 1 and 4
	let mut one_set: HashSet<char> = HashSet::new();
	let mut four_set: HashSet<char> = HashSet::new();

	// Create mapping
	for string in patterns.iter() {
		match string.len() {
			2 => {
				mapping.insert(string, 1);
				one_set = string.chars().collect();
			},
			3 => {
				mapping.insert(string, 7);
			},
			4 => {
				mapping.insert(string, 4);
				four_set = string.chars().collect();
			},
			5 => {
				// Patterns using 5 segments (=2, =3, =5)
				str_len_5.push(string);
				set_mappings.insert(string, string.chars().collect());
			},
			6 => {
				// Patterns using 6 segments (=0, =6, =9)
				str_len_6.push(string);
				set_mappings.insert(string, string.chars().collect());
			},
			7 => {
				mapping.insert(string, 8);
			}
			_ => {
				println!("String could not be matched!");
			},
		}
	}

	// All patterns left use 5 (=2, =3, =5) or 6 (=0, =6, =9) segments
	for candidate in str_len_5 {
		if set_mappings[candidate].intersection(&one_set).collect::<HashSet<_>>().len() == 1 {
			if set_mappings[candidate].intersection(&four_set).collect::<HashSet<_>>().len() == 2 {
				mapping.insert(candidate, 2);
			} else {
				mapping.insert(candidate, 5);
			}
		} else {
			mapping.insert(candidate, 3);
		}
	}

	for candidate in str_len_6 {
		if set_mappings[candidate].intersection(&one_set).collect::<HashSet<_>>().len() == 1 {
			mapping.insert(candidate, 6);
		} else if set_mappings[candidate].intersection(&four_set).collect::<HashSet<_>>().len() == 3 {
			mapping.insert(candidate, 0);
		} else {
			mapping.insert(candidate, 9);
		}
	}

	mapping
}

fn apply_mapping(patterns: &[String], mapping: HashMap<&String, u32>) -> Vec<u32> {
	patterns.iter()
		.map(|p| mapping[p])
		.collect()
}
