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

fn parse_min_max_count(input: &str) -> (u32, u32)
{
	let bounds: Vec<&str> = input.split("-").collect();
	if bounds.len() != 2
	{
		panic!("Invalid password entry");
	}

	let min: u32 = bounds[0].parse().unwrap();
	let max: u32 = bounds[1].parse().unwrap();

	return (min, max);
}

fn parse_required_character(input: &str) -> char
{
	return input.trim().chars().nth(0).unwrap();
}

fn validate_password(input: &str, required_character: char, min: u32, max: u32) -> bool
{
	let count: u32 = input.matches(required_character).count() as u32;

	return count >= min && count <= max;
}

fn count_valid_passwords(input: &Vec<String>) -> u32
{
	let mut result: u32 = 0;

	for line in input
	{
		let s: Vec<&str> = line.split(" ").collect();
		if s.len() != 3
		{
			panic!("Invalid password entry");
		}

		let(min, max) = parse_min_max_count(s[0]);

		// println!("Min {} Max {}", min, max);

		let required_character = parse_required_character(s[1]);

		// println!("Required character {}", required_character);

		let valid_password = validate_password(s[2], required_character, min, max);
		if valid_password
		{
			result += 1;
		}
		// println!("Password valid {}", valid_password);
	}
	return result;
}


fn main()
{
	let args : Vec<String> = env::args().collect();

	if args.len() < 2
	{
		panic!("Not enough arguments")
	}

	// for arg in args.iter()
	// {
	// 	println!("{}", arg);
	// }

	let input_array = read_file_as_vector(&args[1]);
	// println!("{:?}", input_array);

	let result = count_valid_passwords(&input_array);

	println!("Valid passwords: {}", result);

	// let result_pair = find_pair(&input_array, 2020).unwrap();
	// println!("{:?}", result_pair);

	// let result = result_pair.0 * result_pair.1;
	// println!("Result: {}", result);
}
