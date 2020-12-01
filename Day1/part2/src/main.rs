use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn read_file_as_vector(filepath: &str) -> Vec<i64>
{
	let mut result: Vec<i64> = Vec::new();
	let f = File::open(filepath).unwrap();
	for line in io::BufReader::new(f).lines()
	{
		let line_str = line.unwrap();
		let number: i64 = line_str.parse().unwrap();
		result.push(number);
	}
	return result;
}

fn find_pair(input: &Vec<i64>, sum: i64) -> Result<(i64, i64, i64), bool>
{
	
	let input_slice = &input[0..input.len() - 2];

	for (i, x) in input_slice.iter().enumerate() 
	{
		let mut s: Vec<i64> = Vec::new();
		let current_sum = sum - x;

		let inner_slice = &input[i + 1..input.len()];
		for y in inner_slice.iter()
		{
			if s.contains(&(current_sum - y))
			{
				return Ok((*x, *y, current_sum - y));
			}
			s.push(*y);
		}
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

	let result = result_pair.0 * result_pair.1 * result_pair.2;
	println!("Result: {}", result);
}
