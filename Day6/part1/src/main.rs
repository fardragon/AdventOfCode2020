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

fn parse_input(input: &Vec<String>) -> Vec<String>
{
	let mut result = Vec::new();
	let mut tmp = String::new();
	for line in input
	{
		if line.is_empty()
		{
			result.push(tmp.clone());
			tmp.clear();
		}
		else
		{
			tmp += line
		}
	}

	if !tmp.is_empty()
	{
		result.push(tmp);
	}
	return result;
}

fn count_unique(group: &String) -> u16
{
	let mut as_vector: Vec<_> = group.as_bytes().iter().collect();
	as_vector.sort_unstable();
	as_vector.dedup();

	return as_vector.len() as u16;
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
	// for row in &parsed_input
	// {
	// 	println!("{:?}", row);
	// }

	let results : Vec<u16> = parsed_input.iter().map(count_unique).collect();
	let sum: u16 = results.iter().sum();
	println!("Result: {:?}", sum);
}
