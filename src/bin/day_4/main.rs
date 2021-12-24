use std::fmt;

use helper::lines_from_file;

const BOARD_SIZE: usize = 5;

fn main() {
	// --- Part One ---
	println!("--- Part One ---");
	// Read file and convert to u32 vector and 2d-vector for boards
	let unparsed_input = lines_from_file("src/bin/day_4/day_4.txt");
	let unparsed_numbers = &unparsed_input[0];
	let unparsed_boards: Vec<&String> = unparsed_input[2..].iter()
		.filter(|s| !s.is_empty())
		.collect();

	// Ensure that number of lines is dividble by the board size
	assert!(unparsed_boards.len() % BOARD_SIZE == 0);

	// Extract numbers to pull
	let numbers: Vec<u32> = unparsed_numbers.split(",")
		.filter_map(|n| n.parse().ok())
		.collect();

	// Extract boards
	let mut boards: Vec<BingoBoard> = unparsed_boards.chunks(BOARD_SIZE)
		.map(|c| create_board(c, BOARD_SIZE))
		.collect();

	let mut winner_found = false;
	let mut winners_index: Vec<usize> = Vec::new();
	let mut last_number: Option<u32> = None;

	// Pull numbers and check for winner (also contains part two)
	for number in &numbers {
		last_number = Some(*number);
		println!("Now marking number: {}", number);

		// Mark number on all boards
		boards = boards.iter_mut()
			.map(|b| mark_number(b, number))
			.collect();

		// Get winner (part one)
		if !winner_found {
			if let Some(winner) = boards.iter().find(|b| has_won(b)) {
				winner_found = true;
				println!("Winner found!");

				let sum: u32 = winner.state.iter()
					.map(|l| l.iter()
						.filter_map(|e| match e {
							BingoEntry::Entry(n) => Some(*n),
							_ => None,
						}).sum::<u32>()
					).sum();

				println!("First board sum: {}", sum);
				println!("Result (part one): {}", sum * number);
			}
		}

		// Mark winners
		for (index, board) in boards.iter().enumerate() {
			if has_won(board) {
				if !winners_index.contains(&index) {
					winners_index.push(index);
				}
			}
		}

		// Stop early when everyone has won
		if winners_index.len() == boards.len() {
			break;
		}
	}

	if let Some(last_winner_index) = winners_index.pop() {
		let sum: u32 = boards[last_winner_index].state.iter()
		.map(|l| l.iter()
			.filter_map(|e| match e {
				BingoEntry::Entry(n) => Some(*n),
				_ => None,
			}).sum::<u32>()
		).sum();

		// --- Part Two ---
		println!("\n--- Part Two ---");
		println!("Last board sum: {}", sum);
		println!("Result (part two): {}", sum * last_number.unwrap());
	}
}

enum BingoEntry {
	Entry(u32),
	Hit,
}

// Representation of Entry
impl fmt::Display for BingoEntry {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			// TODO: Change this ugly formatting
			BingoEntry::Entry(n) => write!(f, "{:^3}", n),
			BingoEntry::Hit => write!(f, " X ")
		}
    }
}

struct BingoBoard {
	state: Vec<Vec<BingoEntry>>,
	size: usize,
}

// Representation of Board
impl fmt::Display for BingoBoard {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		// Iterator over all Entries
		let formatted_string: String = self.state.iter()
			.map(|l| l.iter()
				.map(|e| format!("{}", e))
				.collect::<Vec<String>>()
				.join(" ")
			)
			.collect::<Vec<String>>()
			.join("\n");

		write!(f, "{}", formatted_string)
	}
}

// Create a board from a reference to (multiple) lines which represent a board
fn create_board(board_string: &[&String], size: usize) -> BingoBoard {
	assert_eq!(BOARD_SIZE, board_string.len());

	let state: Vec<Vec<BingoEntry>> = board_string.iter()
		.map(|l| l.split(" ")
			.filter_map(|n| match n.parse() {
				Ok(n) => Some(BingoEntry::Entry(n)),
				Err(_) => None,
			}).collect())
		.collect();

	BingoBoard {
		state,
		size,
	}
}

// Mark number on given board
fn mark_number(board: &mut BingoBoard, number: &u32) -> BingoBoard {
	BingoBoard {
		state: board.state.iter_mut()
			.map(|l| l.into_iter()
				.map(|e| match e {
					BingoEntry::Entry(n) => {
						if n == number {
							BingoEntry::Hit
						} else {
							BingoEntry::Entry(*n)
						}
					},
					BingoEntry::Hit => BingoEntry::Hit
				}).collect::<Vec<BingoEntry>>())
			.collect::<Vec<Vec<BingoEntry>>>(),
		size: board.size,
	}
}

// Returns true if given bingo board has won
fn has_won(board: &BingoBoard) -> bool {
	// Mark all boards in the beginning
	let mut rows: Vec<bool> = vec![true; board.size];
	let mut columns: Vec<bool> = vec![true; board.size];

	// Remove every row and column containing a "Miss" entry
	for (i, row) in board.state.iter().enumerate() {
		for (j, entry) in row.iter().enumerate() {
			if let BingoEntry::Entry(_) = entry {
				rows[i] = false;
				columns[j] = false;
			}
		}
	}

	// When any row is still marked it only contains Hit entries
	if rows.iter().any(|&v| v) || columns.iter().any(|&v| v) {
		true
	} else {
		false
	}
}
