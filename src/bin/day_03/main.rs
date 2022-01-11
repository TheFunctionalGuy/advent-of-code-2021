use helper::lines_from_file;

fn main() {
	// --- Part One ---
	println!("--- Part One ---");
	// Read file and convert to u32 vector
	let unparsed_input = lines_from_file("src/bin/day_03/day_03.txt");
	let binary_numbers: Vec<u32> = unparsed_input.iter()
		.filter_map(|line| u32::from_str_radix(line, 2).ok())
		.collect();

	assert_eq!(binary_numbers.len(), 1000);

	// Set number of bits
	let num_bits = 12;

	// Extract most common bits
	let mut mask: u32 = 0b1;
	let mut result_bits: u32 = 0;

	for i in 0..num_bits {
		let mut zeros: u32 = 0;
		let mut ones: u32 = 0;
		println!("Now determining most common bit at position: {}", i + 1);
		println!("Mask: {:012b}", mask);

		for number in &binary_numbers {
			if number & mask != 0 {
				ones += 1
			} else {
				zeros += 1
			}
		}

		if ones > zeros {
			result_bits |= mask;
		}

		mask <<= 1;
	}

	// Gamma rate are most common bits
	let gamma_rate: u32 = result_bits;
	// Epsilon is the reverse
	let epsilon_rate: u32 = !result_bits & 0b111111111111;

	println!("Gamma rate:   {:012b} -> {}", gamma_rate, gamma_rate);
	println!("Epsilon rate: {:012b} -> {}", epsilon_rate, epsilon_rate);

	println!("Power consumption of submarine: {}", gamma_rate * epsilon_rate);


	// --- Part Two ---
	println!("\n--- Part Two ---");
	mask = 0b100000000000;
	let mut oxygen_generator_candidates: Vec<u32> = binary_numbers.clone();
	let mut co2_scrubber_rating_candidates: Vec<u32> = binary_numbers;

	for _ in 0..num_bits {
		let mut ones: Vec<u32> = Vec::new();
		let mut zeros: Vec<u32> = Vec::new();

		for candidate in &oxygen_generator_candidates {
			if candidate & mask != 0 {
				ones.push(*candidate);
			} else {
				zeros.push(*candidate);
			}
		}

		if ones.len() >= zeros.len() {
			oxygen_generator_candidates = ones;
		} else {
			oxygen_generator_candidates = zeros;
		}

		if oxygen_generator_candidates.len() == 1 {
			break;
		}

		mask >>= 1;
	}

	mask = 0b100000000000;
	for _ in 0..num_bits {
		let mut ones: Vec<u32> = Vec::new();
		let mut zeros: Vec<u32> = Vec::new();

		for candidate in &co2_scrubber_rating_candidates {
			if candidate & mask != 0 {
				ones.push(*candidate);
			} else {
				zeros.push(*candidate);
			}
		}

		if zeros.len() <= ones.len() {
			co2_scrubber_rating_candidates = zeros;
		} else {
			co2_scrubber_rating_candidates = ones;
		}

		if co2_scrubber_rating_candidates.len() == 1 {
			break;
		}

		mask >>= 1;
	}

	println!("Final oxygen generator rating: {}", oxygen_generator_candidates[0]);
	println!("Final co2 scrubber rating: {}", co2_scrubber_rating_candidates[0]);
	println!("Life support rating: {}", oxygen_generator_candidates[0] * co2_scrubber_rating_candidates[0]);
}
