use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn read_file_as_vector(filepath: &str) -> Vec<u32>
{
	let mut result: Vec<u32> = Vec::new();
	let f = File::open(filepath).unwrap();
	for line in io::BufReader::new(f).lines()
	{
		let line_str = line.unwrap();
		let number: u32 = line_str.parse().unwrap();
		result.push(number);
	}
	return result;
}

fn find_pair(input: &Vec<u32>, sum: u32) -> Result<(u32, u32), bool>
{
	let mut s: Vec<u32> = Vec::new();

	for i in input
	{
		let diff: u32 = sum - i;
		if s.contains(&diff)
		{
			return Ok((*i, diff))
		}
		s.push(*i);
	}

	return Err(true);
}

fn main()
{
	let args : Vec<String> = env::args().collect();

	if args.len() < 2
	{
		panic!("Not enough arguments")
	}

	println!("Hello, world! {}", args.len());

	for arg in args.iter()
	{
		println!("{}", arg);
	}

	let input_array = read_file_as_vector(&args[1]);
	println!("{:?}", input_array);

	let result_pair = find_pair(&input_array, 2020).unwrap();
	println!("{:?}", result_pair);

	let result = result_pair.0 * result_pair.1;
	println!("Result: {}", result);
}
