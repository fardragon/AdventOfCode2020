use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;


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
			tmp += " ";
			tmp += line
		}
	}

	if !tmp.is_empty()
	{
		result.push(tmp);
	}
	return result;
}

fn parse_passwords(input: &Vec<String>) -> Vec<HashMap<&str, &str>>
{
	let mut result = Vec::new();
	for password in input
	{
		let mut pass_entry = HashMap::new();
		let elements = password.split_ascii_whitespace();
		for element in elements
		{
			let parts: Vec<&str> = element.split(':').collect();
			if parts.len() != 2
			{
				panic!("Invalid password entry");
			}
			pass_entry.insert(parts[0], parts[1]);
		}
		result.push(pass_entry);
	}

	return result;
}

fn validate_password(input: &HashMap<&str, &str>) -> bool
{
	let required_keys = vec!("byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid");
	for key in required_keys
	{
		if !input.contains_key(key)
		{
			return false;
		}
	}
	return true;
}

fn validate_passwords(input : &Vec<HashMap<&str, &str>>) -> u32
{
	let mut result: u32 = 0;
	for password in input
	{
		if validate_password(password)
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

	let input = read_file_as_vector(&args[1]);
	let parsed_input = parse_input(&input);
	let passwords = parse_passwords(&parsed_input);
	for row in &passwords
	{
		println!("{:?}", row);
	}

	let result = validate_passwords(&passwords);
	println!("Result: {}", result);

}
