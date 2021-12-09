use helper::lines_from_file;

fn main() {
	// --- Part One ---
	// Read file and parse commands to tuples
	let unparsed_input = lines_from_file("src/bin/day_2/day_2.txt");
	let commands = parse_commands(&unparsed_input);

	assert_eq!(commands.len(), 1000);

	let mut horizontal_position: u32 = 0;
	let mut depth: u32 = 0;

	// Execute all commands
	for (command, unit) in commands {
		match command {
			Command::Forward => {
				horizontal_position += unit;
			},
			Command::Down => {
				depth += unit;
			},
			Command::Up => {
				depth -= unit;
			}
		}
	}

	println!("Depth: {}", depth);
	println!("Horizontal position: {}", horizontal_position);
	println!("Final product: {}", depth * horizontal_position);
}

enum Command {
	Forward,
	Down,
	Up,
}

// Parse commands by unpacking strings to tuples of command and unit
fn parse_commands(unparsed_commands: &Vec<String>) -> Vec<(Command, u32)> {
	let parsed_commands = unparsed_commands.iter()
		.filter_map(|command_string| fetch_command(command_string))
		.collect();

	parsed_commands
}

// Fetch a single command by splitting string into tuple of command and unit
fn fetch_command(command_string: &String) -> Option<(Command, u32)> {
	let splits: Vec<_> = command_string.split(" ").collect();

	let command: Option<Command> = match splits[0] {
		"forward" => Some(Command::Forward),
		"down" => Some(Command::Down),
		"up" => Some(Command::Up),
		_ => None
	};

	let unit: u32 = splits[1].parse().unwrap();

	match command {
		Some(c) => Some((c, unit)),
		None => None
	}
}