use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Instruction {
	Bitmask(Vec<(u64, bool)>),
	Write(u64, u64)
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

fn parse_input(input: &Vec<String>) -> Vec<Instruction>
{
		let mut result = Vec::new();
		for line in input
		{
			let parts: Vec<_> = line.split("=").collect();
			let ins_name = parts[0].trim();

			result.push(match ins_name
			{
				"mask" => 
				{
					let mut mask: Vec<(u64, bool)> = Vec::new();
					for (ix, val) in parts[1].trim().chars().enumerate()
					{
						let pos = parts[1].trim().len() - ix - 1;
						match val
						{
							'X' => {},
							'1' => {mask.push((pos as u64, true))},
							'0' => {mask.push((pos as u64, false))},
							_ => panic!("Invalid bitmask: {}", parts[1])
						}
					}
					Instruction::Bitmask(mask)
				},
				_ =>
				{
					if ins_name.starts_with("mem")
					{
						let write_parts = parts[0].trim().split("[").collect::<Vec<_>>();
						let mut write_addr = write_parts[1].to_string();
						write_addr.pop();
						Instruction::Write(write_addr.parse::<u64>().unwrap(), parts[1].trim().parse::<u64>().unwrap())
					}
					else
					{
						panic!("Invalid instruction: {}", ins_name);
					}
				}
			})
		}
		return result;
}

struct Computer
{
	memory: HashMap<u64, u64>,
	instructions: Vec<Instruction>,
	active_bitmask: Vec<(u64, bool)>
}

impl Computer 
{
	fn apply_bitmask_to_value(&self, input: u64) -> u64
	{
		let mut result = input;
		// println!("Input: {:b}", result);
		for (pos, bit) in &self.active_bitmask
		{
			// println!("Result: {:b}", result);
			match bit
			{
				true =>
				{
					let mask: u64 = 1 << pos;
					// println!("Mask: {:b}", mask);
					result |= mask;
				}
				false =>
				{
					let mask: u64 = !(1 << pos);
					// println!("Mask: {:b}", mask);
					result &= mask;
				}
			}
		}
		// println!("Result: {:b}", result);
		return result;
	}

	fn run(&mut self)
	{
		for ins in &self.instructions
		{
			match ins
			{
				Instruction::Bitmask(x) => self.active_bitmask = x.clone(),
				Instruction::Write(addr, val) =>
				{
					let processed_val = self.apply_bitmask_to_value(*val);
					let entry = self.memory.entry(*addr).or_insert(0);
					*entry = processed_val;
				}
			}
		}
	}

	fn sum_memory(&self) -> u64
	{
		let mut result = 0;
		for (_, val) in &self.memory
		{
			result += val;
		}

		return result;
	}
}

fn find_result(input: Vec<Instruction>) -> u64
{
	let mut computer: Computer = Computer{memory: HashMap::new(), instructions: input, active_bitmask: Vec::new()};
	computer.run();

	return computer.sum_memory();
}


fn main()
{
		let args : Vec<String> = env::args().collect();

		if args.len() < 2
		{
				panic!("Not enough arguments")
		}

		let input_data = read_file_as_vector(&args[1]);

		let parsed_input = parse_input(&input_data);
		println!("Parsed input: {:?}", parsed_input);
		let result = find_result(parsed_input);
		println!("Result: {}", result);
}
