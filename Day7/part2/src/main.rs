use std::env;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Bag
{
	colour: String,
	content: Vec<(String, u32)>
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

fn parse_input(input: &Vec<String>) -> Vec<Bag>
{
	let mut result: Vec<Bag> = Vec::new();

	for line in input
	{
		let mut tmp: Bag = Bag{colour:"".to_string(), content:Vec::new()};
		let parts: Vec<&str> = line.split("bags contain").collect();
		tmp.colour = parts[0].trim().to_string();
	
		if !parts[1].contains(&"no other bags")
		{
			let contents: Vec<_> = parts[1].split(",").collect();
			for bag in contents
			{
				let mut tmp_bag: (String, u32) = ("".to_string(), 0);
				let bag_parts: Vec<_> = bag.trim().split(" ").collect();
				tmp_bag.1 = bag_parts[0].parse().unwrap();
				tmp_bag.0 = format!("{} {}", bag_parts[1], bag_parts[2]).to_string();
				tmp.content.push(tmp_bag);
			}
		}
		result.push(tmp);
	}
	return result;
}

fn count_bag_content(bag_rules: &Vec<Bag>, target_bag_name: &str) -> u32
{
	let target_bag = bag_rules.iter().find( |&x| x.colour == target_bag_name).unwrap();

	let mut result = 1;
	for content in &target_bag.content
	{
		let content_count = content.1 * count_bag_content(bag_rules, &content.0);
		result += content_count;
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

	// for l in &parsed_input
	// {
	// 	println!("{:?}", l);
	// }

	let result = count_bag_content(&parsed_input, "shiny gold");
	println!("Result: {:?}", result - 1);
}
