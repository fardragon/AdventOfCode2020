use std::env;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug)]
enum Square {
    Empty,
    Tree,
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

fn parse_input(input: &Vec<String>) -> Vec<Vec<Square>>
{
	let mut result = Vec::new();
	for line in input
	{
		let mut row = Vec::new();
		for square in line.chars()
		{
			let square_type = match square
			{
				'.' => Square::Empty,
				'#' => Square::Tree,
				_ => panic!("Unknown square type")
			};
			row.push(square_type)
		}
		result.push(row);
	}
	return result;
}

fn traverse_grid(grid: &Vec<Vec<Square>>) -> u32
{
	let mut result: u32 = 0;
	let mut pos: (usize, usize) = (0, 0);
	let finish: usize = grid.len();
	let row_size: usize = grid[0].len();

	let y_step = 1;
	let x_step = 3;

	while pos.1 < finish
	{
		if let Square::Tree = grid[pos.1][pos.0]
		{
			result +=1;
		}

		pos.1 += y_step;
		pos.0 = (pos.0 + x_step) % row_size;
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

	let input_grid = read_file_as_vector(&args[1]);
	for row in &input_grid
	{
		println!("{:?}", row);
	}

	let parsed_grid = parse_input(&input_grid);
	for row in &parsed_grid
	{
		println!("{:?}", row);
	}

	let result = traverse_grid(&parsed_grid);
	println!("{:?}", result);
	
}
