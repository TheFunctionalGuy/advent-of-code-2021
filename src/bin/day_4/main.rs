use helper::lines_from_file;

fn main() {
	// --- Part One ---
	println!("--- Part One ---");
	// Read file and convert to u32 vector and 2d-vector for boards
	let unparsed_input = lines_from_file("src/bin/day_4/day_4.txt");
	//let mut state = [[0u8; 4]; 6];
	let unparsed_numbers = &unparsed_input[0];

	let numbers: Vec<u32> = unparsed_numbers.split(",")
		.filter_map(|number| number.parse().ok())
		.collect();
	
	for number in &numbers {
		println!("{}", number);
	}

	for (index, line) in unparsed_input.iter().enumerate() {
		println!("{:03}: {}", index, line);
	}
}

fn create_board(board_string: &Vec<String>) {

}

trait BingoBoard {
	fn show(&self) -> String;

}