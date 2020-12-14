use std::env;
use std::fs::File;
use std::ops::{SubAssign, AddAssign};
use std::io::{self, BufRead};

type InsArg = u32;

#[derive(Debug, Clone, Copy)]
enum Instruction {
	N(InsArg),
	S(InsArg),
	E(InsArg),
	W(InsArg),
	L(InsArg),
	R(InsArg),
	F(InsArg)
}

#[derive(Debug, Clone, Copy)]
enum Attitude {
	North,
	East,
	South,
	West
}


impl AddAssign<InsArg> for Attitude
{
	fn add_assign(&mut self, other: InsArg) {
		match self
		{
			Attitude::North =>
			{
				*self = match other
				{
					90 => Attitude::East,
					180 => Attitude::South,
					270 => Attitude::West,
					_ => panic!("Invalid other: {}", other)
				}
			},
			Attitude::East =>
			{
				*self = match other
				{
					90 => Attitude::South,
					180 => Attitude::West,
					270 => Attitude::North,
					_ => panic!("Invalid other: {}", other)
				}
			},
			Attitude::South =>
			{
				*self = match other
				{
					90 => Attitude::West,
					180 => Attitude::North,
					270 => Attitude::East,
					_ => panic!("Invalid other: {}", other)
				}
			},
			Attitude::West =>
			{
				*self = match other
				{
					90 => Attitude::North,
					180 => Attitude::East,
					270 => Attitude::South,
					_ => panic!("Invalid other: {}", other)
				}
			}
		}
	}
}

impl SubAssign<InsArg> for Attitude
{
	fn sub_assign(&mut self, other: InsArg) {
		match self
		{
			Attitude::North =>
			{
				*self = match other
				{
					90 => Attitude::West,
					180 => Attitude::South,
					270 => Attitude::East,
					_ => panic!("Invalid other: {}", other)
				}
			},
			Attitude::East =>
			{
				*self = match other
				{
					90 => Attitude::North,
					180 => Attitude::West,
					270 => Attitude::South,
					_ => panic!("Invalid other: {}", other)
				}
			},
			Attitude::South =>
			{
				*self = match other
				{
					90 => Attitude::East,
					180 => Attitude::North,
					270 => Attitude::West,
					_ => panic!("Invalid other: {}", other)
				}
			},
			Attitude::West =>
			{
				*self = match other
				{
					90 => Attitude::South,
					180 => Attitude::East,
					270 => Attitude::North,
					_ => panic!("Invalid other: {}", other)
				}
			}
		}
	}
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

fn parse_move_arg(input: &str) -> InsArg
{
	return input.parse().unwrap();
}

fn parse_turn_arg(input: &str) -> InsArg
{
	let tmp = input.parse().unwrap();
	match tmp
	{
		90 | 180 | 270 => return tmp,
		_ => panic!("Invalid turn arg: {}", tmp)
	}
}

fn parse_input(input: &Vec<String>) -> Vec<Instruction>
{
		let mut result = Vec::new();
		for line in input
		{
			let ins_name = line.chars().nth(0).unwrap();
			let arg: String = line.chars().skip(1).collect();
			let instruction = match ins_name
			{
				'N' => Instruction::N(parse_move_arg(&arg)),
				'E' => Instruction::E(parse_move_arg(&arg)),
				'S' => Instruction::S(parse_move_arg(&arg)),
				'W' => Instruction::W(parse_move_arg(&arg)),
				'F' => Instruction::F(parse_move_arg(&arg)),
				'L' => Instruction::L(parse_turn_arg(&arg)),
				'R' => Instruction::R(parse_turn_arg(&arg)),
				_ => panic!("Unknown instruction type: {}", ins_name)
			};
			result.push(instruction)
		}
		return result;
}

struct Ship
{
	current_attitude: Attitude,
	current_pos: (i64, i64)
}

impl Ship
{
	fn navigate(&mut self, instruction: Instruction)
	{
		match instruction
		{
			Instruction::N(arg) => self.current_pos.1 += arg as i64,
			Instruction::E(arg) => self.current_pos.0 += arg as i64,
			Instruction::S(arg) => self.current_pos.1 -= arg as i64,
			Instruction::W(arg) => self.current_pos.0 -= arg as i64,
			Instruction::L(arg) => self.current_attitude -= arg,
			Instruction::R(arg) => self.current_attitude += arg,
			Instruction::F(arg) => {
				match self.current_attitude
				{
					Attitude::North => self.current_pos.1 += arg as i64,
					Attitude::East => self.current_pos.0 += arg as i64,
					Attitude::South => self.current_pos.1 -= arg as i64,
					Attitude::West => self.current_pos.0 -= arg as i64,
				}
			}
		}
	}
}

fn navigate_path(instructions: &Vec<Instruction>) -> (i64, i64)
{
	let mut ship = Ship{current_attitude: Attitude::East, current_pos: (0, 0)};
	for instruction in instructions
	{
		ship.navigate(*instruction);
	}

	return ship.current_pos;
}

fn main()
{
		let args : Vec<String> = env::args().collect();

		if args.len() < 2
		{
				panic!("Not enough arguments")
		}

		let input_data = read_file_as_vector(&args[1]);

		let parsed_nav = parse_input(&input_data);

		let final_pos = navigate_path(&parsed_nav);
		println!("{:?}", final_pos);
		println!("Result: {}", final_pos.0.abs() + final_pos.1.abs());
}
