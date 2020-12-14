use std::env;
use std::fs::File;
use std::io::{self, BufRead};





fn read_file_as_vector(filepath: &str) -> Vec<String>
{
		let mut result: Vec<String> = Vec::new();
		let f = File::open(filepath).unwrap();
		for line in io::BufReader::new(f).lines()
		{
				let line_str = line.unwrap();
				result.push(line_str);
		}
		return result;
}



fn parse_input(input: &Vec<String>) -> Option<(u64, Vec<u64>)>
{
	if input.len() != 2
	{
		return None;
	}

	let target_time: u64 = input[0].parse().unwrap();

	let mut available_buses: Vec<u64> = Vec::new();
	for bus in input[1].split(",")
	{
		match bus.parse::<u64>()
		{
			Ok(x) => {available_buses.push(x)},
			Err(e) => {println!("Unrecognized input: {}", e)}
		}
	}

	return Some((target_time, available_buses));
}

fn find_best_bus(input: &(u64, Vec<u64>)) -> (u64, u64, u64)
{
	let (target_time, available_buses) = input;

	let mut tmp_results: Vec<(u64, u64)> = Vec::new();
	for bus in available_buses
	{
		if target_time % bus == 0
		{
			tmp_results.push((*bus, *target_time));
		}
		else
		{
			let arrival_time = ((target_time / bus) + 1) * bus;
			tmp_results.push((*bus, arrival_time));
		}
	}

	let best_bus = tmp_results.iter().min_by_key(|bus| bus.1).unwrap();


	return (best_bus.0 , best_bus.1, ( best_bus.1 - target_time) * best_bus.0);
}


fn main()
{
		let args : Vec<String> = env::args().collect();

		if args.len() < 2
		{
				panic!("Not enough arguments")
		}

		let input_data = read_file_as_vector(&args[1]);

		let parsed_input = parse_input(&input_data).unwrap();
		println!("{:?}", parsed_input);

		let best_bus = find_best_bus(&parsed_input);
		println!("{:?}", best_bus);
}
