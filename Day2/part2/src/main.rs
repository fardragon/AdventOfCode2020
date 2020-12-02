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

fn parse_indices(input: &str) -> (u32, u32)
{
	let bounds: Vec<&str> = input.split("-").collect();
	if bounds.len() != 2
	{
		panic!("Invalid password entry");
	}

	let index1: u32 = bounds[0].parse().unwrap();
	let index2: u32 = bounds[1].parse().unwrap();

	return (index1, index2);
}

fn parse_required_character(input: &str) -> char
{
	return input.trim().chars().nth(0).unwrap();
}

fn validate_password(input: &str, required_character: char, index1: u32, index2: u32) -> bool
{
	let first_char_match = input.chars().nth((index1 - 1) as usize).unwrap() == required_character;
	let second_char_match = input.chars().nth((index2 - 1) as usize).unwrap() == required_character;


	return first_char_match ^ second_char_match;
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

		let(index1, index2) = parse_indices(s[0]);
		let required_character = parse_required_character(s[1]);

		let valid_password = validate_password(s[2], required_character, index1, index2);
		if valid_password
		{
			result += 1;
		}
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
