use std::env;
use std::fs::File;
use std::io::{self, BufRead};


#[derive(Debug)]
enum Instruction
{
	ACC, JMP, NOP
}

#[derive(Debug)]
struct Operation
{
	op: Instruction,
	arg: i32,
	executions: i32
}

#[derive(Debug)]
struct Machine
{
	instruction_pointer: i64,
	instructions: Vec<Operation>,
	accumulator: i64
}

impl Machine
{
	fn run_until_loop(&mut self)
	{
		while self.instructions[self.instruction_pointer as usize].executions == 0
		{
			let ins: &mut Operation = &mut self.instructions[self.instruction_pointer as usize];
			match ins.op {
				Instruction::ACC => {self.accumulator += ins.arg as i64; self.instruction_pointer += 1;},
				Instruction::JMP => {self.instruction_pointer += ins.arg as i64;},
				Instruction::NOP => {self.instruction_pointer += 1;}
			}
			ins.executions += 1;
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

fn parse_input(input: &Vec<String>) -> Vec<Operation>
{
	let mut result: Vec<Operation> = Vec::new();
	for line in input
	{
		let parts: Vec<_> = line.split(" ").collect();
		if parts.len() != 2
		{
			panic!("Invalid input");
		}

		let op = match parts[0] {
			"acc" => Instruction::ACC,
			"jmp" => Instruction::JMP,
			"nop" => Instruction::NOP,
			_ => panic!("Invalid opration")
		};

		let arg: i32 = parts[1].parse().unwrap();
		result.push(Operation{op:op, arg:arg, executions:0});
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
	for l in &parsed_input
	{
		println!("{:?}", l);
	}

	println!("\r\n Running \r\n");
	let mut machine = Machine{instruction_pointer: 0, instructions: parsed_input, accumulator: 0};
	machine.run_until_loop();

	for l in &machine.instructions
	{
		println!("{:?}", l);
	}


	// let result = count_bag_content(&parsed_input, "shiny gold");
	println!("Result: {:?}", machine.accumulator);

}
