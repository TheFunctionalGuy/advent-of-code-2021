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

	// Find all syntax errors
	let mut syntax_error_scores: Vec<u32> = Vec::new();
	let mut incomplete_lines: Vec<Vec<char>> = Vec::new();

	for line in navigation_subsystem {
		if let Some(score) = check_syntax(&line) {
			syntax_error_scores.push(score);
		} else {
			incomplete_lines.push(line);
		}
	}

	println!("Total syntax error score: {}", syntax_error_scores.iter().sum::<u32>());


	// --- Part Two ---
	println!("\n--- Part Two ---");
	let missing_brackets: Vec<Vec<char>> = incomplete_lines.iter()
		.map(|l| complete_line(l))
		.collect();

	let mut autocomplete_scores: Vec<u64> = missing_brackets.iter()
		.map(|v| v.iter()
			.fold(0, |score, bracket| {
				match bracket {
					')' => score * 5 + 1,
					']' => score * 5 + 2,
					'}' => score * 5 + 3,
					'>' => score * 5 + 4,
					_ => score,
				}
			}))
		.collect();

	assert!(autocomplete_scores.len() % 2 == 1);

	// Sort score
	autocomplete_scores.sort_unstable();

	// Calculate index
	let index: usize = autocomplete_scores.len() / 2;
	println!("Middle autocomplete score: {}", autocomplete_scores[index]);
}

fn check_syntax(line: &[char]) -> Option<u32> {
	let mut bracket_stack: Vec<char> = Vec::new();

	for bracket in line {
		if ['(', '[', '{', '<'].contains(bracket) {
			bracket_stack.push(*bracket);
		} else if let Some(pairing_bracket) = bracket_stack.pop() {
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

	None
}

fn complete_line(line: &[char]) -> Vec<char> {
	let mut bracket_stack: Vec<char> = Vec::new();
	let mut missing_brackets: Vec<char> = Vec::new();

	for bracket in line {
		if ['(', '[', '{', '<'].contains(bracket) {
			bracket_stack.push(*bracket);
		} else {
			bracket_stack.pop();
		}
	}

	while let Some(bracket) = bracket_stack.pop() {
		match bracket {
			'(' => missing_brackets.push(')'),
			'[' => missing_brackets.push(']'),
			'{' => missing_brackets.push('}'),
			'<' => missing_brackets.push('>'),
			_ => {},
		}
	}

	missing_brackets
}
