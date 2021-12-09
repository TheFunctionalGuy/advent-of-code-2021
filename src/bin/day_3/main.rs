use helper::lines_from_file;

fn main() {
	// --- Part One ---+
	// Read file and convert to u32 vector
	let unparsed_input = lines_from_file("src/bin/day_3/day_3.txt");
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
			result_bits = result_bits | mask;
		}

		mask = mask << 1;
	}

	// Gamma rate are most common bits
	let gamma_rate: u32 = result_bits;
	// Epsilon is the reverse
	let epsilon_rate: u32 = !result_bits & 0b111111111111; 

	println!("Gamma rate:   {:012b} -> {}", gamma_rate, gamma_rate);
	println!("Epsilon rate: {:012b} -> {}", epsilon_rate, epsilon_rate);

	println!("Power consumption of submarine: {}", gamma_rate * epsilon_rate)
}
