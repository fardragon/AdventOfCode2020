use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum Instruction {
	Bitmask(Vec<(u64, u8)>),
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
					let mut mask: Vec<(u64, u8)> = Vec::new();
					for (ix, val) in parts[1].trim().chars().enumerate()
					{
						let pos = parts[1].trim().len() - ix - 1;
						match val
						{
							'X' => {mask.push((pos as u64, 2))},
							'1' => {mask.push((pos as u64, 1))},
							'0' => {mask.push((pos as u64, 0))},
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
	active_bitmask: Vec<(u64, u8)>,
	bitmask_floating: u32
}

impl Computer 
{
	fn gray_encode(integer: u64) -> u64 
	{
		(integer >> 1) ^ integer
	}

	fn apply_bitmask_to_addr(&self, input: u64) -> Vec<u64>
	{
		let mut result = Vec::new();
		let mut result_count: u64 = 2;
		result_count = result_count.pow(self.bitmask_floating);

		for ix in 0 .. result_count
		{
			let mut tmp_result: u64 = input;
			let mut floating_index = 0;
			let current_result_gray = Computer::gray_encode(ix);
			// println!("Input: {:b}", tmp_result);
			for (pos, val) in &self.active_bitmask
			{
				// println!("Result: {:b}", result);
				match val
				{
					1 =>
					{
						// println!("Setting bit {} to 1", pos);
						let mask: u64 = 1 << pos;
						// println!("Mask: {:b}", mask);
						tmp_result |= mask;
					}
					0 =>
					{
						// println!("Skipping bit {}", pos);
					}
					2 =>
					{
						// println!("Addr: {:b}", tmp_result);
						if (1 << floating_index) & current_result_gray > 0
						{
							// println!("Setting bit {} to 1", pos);
							let mask: u64 = 1 << pos;
							tmp_result |= mask;
						}
						else
						{
							// println!("Setting bit {} to 0", pos);
							let mask: u64 = !(1 << pos);
							tmp_result &= mask;
						}

						floating_index += 1;
					}
					_ => panic!("Invalid bitmask {:?}", self.active_bitmask)
				}
			}
			// println!("Addr: {:b}", tmp_result);
			result.push(tmp_result);
		}
		return result;
	}

	fn run(&mut self)
	{
		for ins in &self.instructions
		{
			match ins
			{
				Instruction::Bitmask(x) => 
				{
					self.active_bitmask = x.clone();
					self.active_bitmask.sort_by_key(|x| x.0);
					self.bitmask_floating = 0;
					for (_, b) in x
					{
						if *b == 2
						{
							self.bitmask_floating += 1;
						}	
					}
				},
				Instruction::Write(addr, val) =>
				{
					let processed_addr = self.apply_bitmask_to_addr(*addr);
					// println!("Addrs {:?}", processed_addr);
					for p_addr in processed_addr
					{

						let entry = self.memory.entry(p_addr).or_insert(0);
						*entry = *val;
					}
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
	let mut computer: Computer = Computer{memory: HashMap::new(), instructions: input, active_bitmask: Vec::new(), bitmask_floating: 0};
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
