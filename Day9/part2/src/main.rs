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

fn find_target_sum(input: &Vec<i64>, preamble_size: i64) -> i64
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

fn find_sum(input: &[i64], sum: i64) -> (bool, i64)
{
	let mut tmp_sum: i64 = 0;
	let mut current_len: i64 = 0;
	for i in input
	{
		current_len += 1;
		tmp_sum += i;
		if tmp_sum < sum
		{
			continue;
		}
		else if tmp_sum == sum
		{
			if current_len >= 1
			{
				return (true, current_len);
			}
			else
			{
				break;
			}
		}
		else
		{
			break;
		}
	}
	return (false, 0);
}

fn find_weakness(input: &Vec<i64>, result: i64) -> (usize, usize)
{

	for starting_pos in 0..input.len()
	{
		let tmp_res = find_sum(&input[starting_pos .. input.len()], result);

		if tmp_res.0
		{
			return (starting_pos, starting_pos + tmp_res.1 as usize)
		}
	}

	panic!("Weakness not found");
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

	let target_sum = find_target_sum(&parsed_input, 25);
	println!("Target sum: {}", target_sum);

	let weakness = find_weakness(&parsed_input, target_sum);
	println!("Weakness: {:?}", weakness);

	let weakness_slice = &parsed_input[weakness.0 .. weakness.1];
	println!("Slice {:?}", weakness_slice);

	let result = weakness_slice.iter().max().unwrap() + weakness_slice.iter().min().unwrap();
	println!("Result {}", result);
}
