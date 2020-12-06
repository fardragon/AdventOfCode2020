use std::env;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct PassengerGroup
{
	answers: Vec<String>
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

fn parse_input(input: &Vec<String>) -> Vec<PassengerGroup>
{
	let mut result: Vec<PassengerGroup> = Vec::new();
	let mut tmp: PassengerGroup = PassengerGroup{answers: Vec::new()};

	for line in input
	{
		if line.is_empty()
		{
			result.push(tmp);
			tmp = PassengerGroup{answers: Vec::new()};
		}
		else
		{
			tmp.answers.push(line.clone());
		}
	}

	if !tmp.answers.is_empty()
	{
		result.push(tmp);
	}
	return result;
}

fn count_common(group: &PassengerGroup) -> u16
{
	if group.answers.len() == 1
	{
		return group.answers.first().unwrap().len() as u16;
	}

	let mut result: u16 = 0;
	'outer: for ch in group.answers.first().unwrap().chars()
	{
		for vec in &group.answers[1..group.answers.len()]
		{
			if !vec.contains(ch)
			{
				continue 'outer;
			}
		}
		result += 1;
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

	let input = read_file_as_vector(&args[1]);
	let parsed_input = parse_input(&input);

	let results : Vec<u16> = parsed_input.iter().map(count_common).collect();
	let sum: u16 = results.iter().sum();
	println!("Result: {:?}", sum);
}
