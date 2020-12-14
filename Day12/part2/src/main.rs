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
	current_pos: (i64, i64),
	current_waypoint: (i64, i64)
}

impl Ship
{
	fn rotate_waypoint_left(&mut self, angle: InsArg)
	{
		self.current_waypoint = match angle
		{
			90 => (-self.current_waypoint.1, self.current_waypoint.0),
			180 => (-self.current_waypoint.0, -self.current_waypoint.1),
			270 => (self.current_waypoint.1, -self.current_waypoint.0),
			_ => panic!("Invalid angle: {}", angle)
		}
	}

	fn rotate_waypoint_right(&mut self, angle: InsArg)
	{
		self.current_waypoint = match angle
		{
			270 => (-self.current_waypoint.1, self.current_waypoint.0),
			180 => (-self.current_waypoint.0, -self.current_waypoint.1),
			90 => (self.current_waypoint.1, -self.current_waypoint.0),
			_ => panic!("Invalid angle: {}", angle)
		}
	}

	fn navigate(&mut self, instruction: Instruction)
	{
		match instruction
		{
			Instruction::N(arg) => self.current_waypoint.1 += arg as i64,
			Instruction::E(arg) => self.current_waypoint.0 += arg as i64,
			Instruction::S(arg) => self.current_waypoint.1 -= arg as i64,
			Instruction::W(arg) => self.current_waypoint.0 -= arg as i64,
			Instruction::L(arg) => self.rotate_waypoint_left(arg),
			Instruction::R(arg) => self.rotate_waypoint_right(arg),
			Instruction::F(arg) => {
				self.current_pos.0 += arg as i64 * self.current_waypoint.0;
				self.current_pos.1 += arg as i64 * self.current_waypoint.1;
			}
		}
	}
}

fn navigate_path(instructions: &Vec<Instruction>) -> (i64, i64)
{
	let mut ship = Ship{current_pos: (0, 0),  current_waypoint: (10, 1)};
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
