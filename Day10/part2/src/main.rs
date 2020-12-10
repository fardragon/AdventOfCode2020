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

fn parse_input(input: &Vec<String>) -> Vec<i64>
{
	let result: Vec<i64> = input.iter().map(|x| x.parse().unwrap()).collect();
	return result;
}

fn count_paths(slice: &[i64]) -> i64
{
	if slice.len() == 0
	{
		return 0;
	}
	else if slice.len() < 3
	{
		return 1;
	}

	let mut paths:i64 = 0;
	paths += count_paths(&slice[1..slice.len()]);
	paths += count_paths(&slice[2..slice.len()]);
	paths += count_paths(&slice[3..slice.len()]);
	return paths;
}

fn calculate_result(input: Vec<i64>) -> i64
{
	let mut start: usize = 0;
	let mut prev: i64 = 0;
	let mut paths: i64 = 1;

	for (i, x) in input.iter().enumerate()
	{
		if (prev + 3) == *x
		{
			let tmp_paths = count_paths(&input[start .. i]);
			paths *= tmp_paths;
			start = i;
		}
		prev = *x;
	}

	return paths;
}

fn main()
{
	let args : Vec<String> = env::args().collect();

	if args.len() < 2
	{
			panic!("Not enough arguments")
	}

	let input = read_file_as_vector(&args[1]);
	let mut parsed_input = parse_input(&input);
	parsed_input.push(0);
	parsed_input.sort();
	parsed_input.push(parsed_input.last().unwrap() + 3);

	let result = calculate_result(parsed_input);
	println!("{}", result);
}
