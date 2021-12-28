use std::cmp::{max, min};

use helper::lines_from_file;

fn main() {
	// --- Part One ---
	println!("--- Part One ---");
	// Read file and convert to vector u32 value
	let unparsed_input = lines_from_file("src/bin/day_7/day_7.txt");
	let horizontal_positions: Vec<u32> = unparsed_input[0].split(',')
		.filter_map(|n| n.parse().ok())
		.collect();

	// Ensure all numbers have been parsed correctly
	assert_eq!(1000, horizontal_positions.len());

	// Get highest posible crab position
	let max_position = horizontal_positions.iter().max().unwrap();

	println!("Highest horizontal position: {}", max_position);

	// Calculate all fuel costs
	let fuel_costs: Vec<u32> = (0..=*max_position).into_iter()
		.map(|p| horizontal_positions.iter()
			.map(|pos| distance(&p, pos))
			.sum())
		.collect();

	for (i, fuel_cost) in fuel_costs.iter().enumerate() {
		println!("Aligning at position {} would cost {} fuel", i, fuel_cost);
	}

	// Print minimal fuel cost
	if let Some((index, _)) = fuel_costs.iter().enumerate().min_by(|&(_, x), &(_, y)| x.cmp(y)) {
		println!("Index for minimal fuel cost is: {}", index);
		println!("Minimal fuel cost is: {}", fuel_costs[index]);
	}


	// --- Part Two ---
	println!("\n--- Part Two ---");

	// Calculate fuel cost according to new rules
	let new_fuel_costs: Vec<u32> = (0..=*max_position).into_iter()
		.map(|p| horizontal_positions.iter()
			.map(|pos| distance(&p, pos))
			.map(|c| (c * (c + 1)) / 2)
			.sum())
		.collect();

	for (i, fuel_cost) in new_fuel_costs.iter().enumerate() {
		println!("Aligning at position {} would cost {} fuel", i, fuel_cost);
	}

	// Print new minimal fuel cost
	if let Some((index, _)) = new_fuel_costs.iter().enumerate().min_by(|&(_, x), &(_, y)| x.cmp(y)) {
		println!("Index for minimal fuel cost is: {}", index);
		println!("Minimal fuel cost is: {}", new_fuel_costs[index]);
	}
}

fn distance(a: &u32, b: &u32) -> u32 {
	max(a, b) - min(a, b)
}
