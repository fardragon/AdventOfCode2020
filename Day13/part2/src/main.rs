use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::cmp::{max, min};
 
fn gcd(a: u64, b: u64) -> u64 {
    match ((a, b), (a & 1, b & 1)) {
        ((x, y), _) if x == y => y,
        ((0, x), _) | ((x, 0), _) => x,
        ((x, y), (0, 1)) | ((y, x), (1, 0)) => gcd(x >> 1, y),
        ((x, y), (0, 0)) => gcd(x >> 1, y >> 1) << 1,
        ((x, y), (1, 1)) => {
            let (x, y) = (min(x, y), max(x, y));
            gcd((y - x) >> 1, x)
        }
        _ => unreachable!(),
    }
}
 
fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn lcm_slice(input: &[u64]) -> u64
{
	let mut result: u64 = 1;
	for val in input
	{
		result = lcm(result, *val);
	}
	return result;
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

fn parse_input(input: &Vec<String>) -> Option<Vec<(u64, u64)>>
{
	if input.len() != 2
	{
		return None;
	}

	let mut available_buses: Vec<(u64, u64)> = Vec::new();
	let mut offset: u64 = 0;
	for bus in input[1].split(",")
	{
		match bus.parse::<u64>()
		{
			Ok(x) => {available_buses.push((x, offset))},
			Err(e) => {println!("Unrecognized input: {}", e)}
		}
		offset += 1;
	}

	available_buses.sort_by_key(|bus| bus.1);

	return Some(available_buses);
}

fn can_depart_at(bus: u64, ts: u64) -> bool
{
	return ts % bus == 0;
}

fn find_departure_time(bus1:(u64, u64), step: u64, start_ts: u64) -> u64
{
	let mut ts = start_ts;
	loop
	{
		if can_depart_at(bus1.0, ts + bus1.1)
		{
			return ts;
		}
		ts += step;
	}
}

fn find_result(input: &Vec<(u64, u64)>) -> u64
{

	let mut steps: Vec<u64> = vec![1];
	let mut current_ts: u64 = 1;
	for bus in input
	{
		let step = lcm_slice(&steps);
		current_ts = find_departure_time(*bus, step, current_ts);
		steps.push(bus.0);
	}
	return current_ts;
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

		let result = find_result(&parsed_input);
		println!("{:?}", result);
}
