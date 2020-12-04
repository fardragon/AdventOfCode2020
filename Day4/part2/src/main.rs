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

fn validate_byr(input: &HashMap<&str, &str>) -> bool
{
	if !input.contains_key("byr")
	{
		return false;
	}

	let val = input["byr"];
	if val.len() != 4
	{
		return false;
	}

	let year: u32 = val.parse().unwrap();
	if (year < 1920) || (year > 2002)
	{
		return false;
	}
	

	return true;
}

fn validate_iyr(input: &HashMap<&str, &str>) -> bool
{
	if !input.contains_key("iyr")
	{
		return false;
	}

	let val = input["iyr"];
	if val.len() != 4
	{
		return false;
	}

	let year: u32 = val.parse().unwrap();
	if (year < 2010) ||( year > 2020)
	{
		return false;
	}
	
	return true;
}

fn validate_eyr(input: &HashMap<&str, &str>) -> bool
{
	if !input.contains_key("eyr")
	{
		return false;
	}

	let val = input["eyr"];
	if val.len() != 4
	{
		return false;
	}

	let year: u32 = val.parse().unwrap();
	if (year < 2020) || (year > 2030)
	{
		return false;
	}
	
	return true;
}

fn validate_hgt(input: &HashMap<&str, &str>) -> bool
{
	if !input.contains_key("hgt")
	{
		return false;
	}

	let val = input["hgt"];

	if val.contains("cm")
	{
		let hgt: u32 = val[0..val.len()-2].parse().unwrap();
		if hgt < 150 || hgt > 193
		{
			return false;
		}
	}
	else if val.contains("in")
	{
		let hgt: u32 = val[0..val.len()-2].parse().unwrap();
		if hgt < 59 || hgt > 76
		{
			return false;
		}
	}
	else
	{
		return false;
	}

	return true;
}

fn validate_hcl(input: &HashMap<&str, &str>) -> bool
{
	if !input.contains_key("hcl")
	{
		return false;
	}

	let val = input["hcl"];
	if val.len() != 7
	{
		return false;
	}

	if val.chars().nth(0).unwrap() != '#'
	{
		return false;
	}

	for ch in val[1..val.len()].chars()
	{
		if !ch.is_ascii_hexdigit()
		{
			return false;
		}
	}
	
	return true;
}

fn validate_ecl(input: &HashMap<&str, &str>) -> bool
{
	if !input.contains_key("ecl")
	{
		return false;
	}

	let val = input["ecl"];
	if val.len() != 3
	{
		return false;
	}

	return match val
	{
		"amb" => true,
		"blu" => true,
		"brn" => true,
		"gry" => true,
		"grn" => true,
		"hzl" => true,
		"oth" => true,
		_ => false
	}
}

fn validate_pid(input: &HashMap<&str, &str>) -> bool
{
	if !input.contains_key("pid")
	{
		return false;
	}

	let val = input["pid"];
	if val.len() != 9
	{
		return false;
	}

	for ch in val.chars()
	{
		if !ch.is_ascii_digit()
		{
			return false;
		}
	}
	
	return true;
}




fn validate_password(input: &HashMap<&str, &str>) -> bool
{
	let required_keys: Vec<fn( &HashMap<&str, &str>) -> bool> = vec![validate_byr, validate_iyr, validate_eyr, validate_hgt, validate_hcl, validate_ecl, validate_pid];
	for validator in required_keys
	{
		if !validator(input)
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
			println!("Valid")
		}
		else {
			println!("Invalid")
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
