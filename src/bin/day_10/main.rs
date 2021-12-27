use helper::lines_from_file;

fn main() {
	// --- Part One ---
	println!("--- Part One ---");
	// Read file and convert to vector of char values
	let unparsed_input = lines_from_file("src/bin/day_10/day_10.txt");

	let navigation_subsystem: Vec<Vec<char>> = unparsed_input.iter()
		.map(|l| l.chars().collect())
		.collect();
	
	// Ensure all numbers have been parsed correctly
	assert_eq!(98, navigation_subsystem.len());

	let syntax_error_scores: Vec<u32> = navigation_subsystem.iter()
		.filter_map(|l| check_syntax(l))
		.collect();
	
	println!("Total syntax error score: {}", syntax_error_scores.iter().sum::<u32>());
}

fn check_syntax(line: &Vec<char>) -> Option<u32> {
	let mut bracket_stack: Vec<char> = Vec::new();

	for bracket in line {
		if ['(', '[', '{', '<'].contains(bracket) {
			bracket_stack.push(*bracket);
		} else {
			if let Some(pairing_bracket) = bracket_stack.pop() {
				match (pairing_bracket, bracket) {
					// Check for matching brackets
					('(', ')') => {},
					('[', ']') => {},
					('{', '}') => {},
					('<', '>') => {},
					// Check for non-matching brackets
					(_, ')') => {
						return Some(3);
					},
					(_, ']') => {
						return Some(57);
					},
					(_, '}') => {
						return Some(1197);
					},
					(_, '>') => {
						return Some(25137);
					},
					_ => {}
				}
			}
		}
	}

	None
}