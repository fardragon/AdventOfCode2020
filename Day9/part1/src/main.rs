use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn find_pair(input: &[i64], sum: &i64) -> bool
{
	let mut s: Vec<i64> = Vec::new();
	for i in input
	{
		let diff: i64 = sum - i;
		if s.contains(&diff)
		{
			return true;
		}
		s.push(*i);
	}

	return false;
}

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

fn parse_input(input: &Vec<String>) -> Vec<i64>
{
	let result: Vec<i64> = input.iter().map(|x| x.parse().unwrap()).collect();
	return result;
}

fn find_result(input: &Vec<i64>, preamble_size: i64) -> i64
{
	let mut target_window = &input[0..(preamble_size as usize)];
	let mut preamble_offset = 0;
	let data_slice = &input[(preamble_size as usize) .. input.len()];
	for number in data_slice
	{
		if !find_pair(target_window, number)
		{
			return *number;
		}
		else
		{
			preamble_offset += 1;
			target_window = &input[preamble_offset as usize .. ((preamble_size + preamble_offset) as usize)];
		}
	}
	panic!("Result not found");
}

fn main()
{
	let args : Vec<String> = env::args().collect();

	if args.len() < 2
	{
			panic!("Not enough arguments")
	}

	let input = read_file_as_vector(&args[1]);
	let parsed_input = parse_input(&input);
	// for l in &parsed_input
	// {
	// 	println!("{:?}", l);
	// }

	let result = find_result(&parsed_input, 25);
	println!("Result: {}", result);
}
