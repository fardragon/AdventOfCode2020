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

fn parse_seat_row(seat_id: &str) -> u8
{
	let mut result: u8 = 0;
	let mut pos: u8 = 64;
	for ch in seat_id[0..7].chars()
	{
		if ch == 'B'
		{
			result += pos;
		}
		pos /=2;
	}
	return result;
}

fn parse_seat_column(seat_id: &str) -> u8
{
	let mut result: u8 = 0;
	let mut pos: u8 = 4;
	for ch in seat_id[7..10].chars()
	{
		if ch == 'R'
		{
			result += pos;
		}
		pos /= 2;
	}
	return result;
}

fn parse_seat_id(seat_id: &String) -> u16
{
	let row_id = parse_seat_row(seat_id);
	let column_id = parse_seat_column(seat_id);

	return row_id as u16 * 8 + column_id as u16;
}


fn main()
{
	let args : Vec<String> = env::args().collect();

	if args.len() < 2
	{
		panic!("Not enough arguments")
	}

	let input = read_file_as_vector(&args[1]);
	
	let mut results : Vec<u16> = input.iter().map(parse_seat_id).collect();
	results.sort();

	for p in results.windows(2)
	{
		if (p[1] - p[0]) > 1
		{
			println!("Result: {:?}", (p[1] + p[0]) / 2);
		}
		
	}
}
