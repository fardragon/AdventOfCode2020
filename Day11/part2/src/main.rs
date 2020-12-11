use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy)]
enum Square {
	EmptyChair,
	TakenChair,
	Floor,
}

impl fmt::Display for Square
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Square::EmptyChair => write!(f, "L"),
			Square::TakenChair => write!(f, "#"),
			Square::Floor => write!(f, "."),
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
								'.' => Square::Floor,
								'L' => Square::EmptyChair,
								'#' => panic!("Occupied chair in input"),
								_ => panic!("Unknown square type")
						};
						row.push(square_type)
				}
				result.push(row);
		}
		return result;
}

fn find_seat_in_direction(input: &Vec<Vec<Square>>, start_pos: (i64, i64), direction: (i64, i64), dimensions: (i64, i64)) -> Option<(i64, i64)>
{
	let mut current_pos = start_pos;
	loop
	{
		current_pos.0 += direction.0;
		current_pos.1 += direction.1;

		if (current_pos.0 < 0) || (current_pos.1 < 0)
		{
			return None;
		}

		if (current_pos.0 >= dimensions.0) || (current_pos.1 >= dimensions.1)
		{
			return None;
		}

		match input[current_pos.1 as usize][current_pos.0 as usize] 
		{
			Square::EmptyChair => return Some(current_pos),
			Square::TakenChair => return Some(current_pos),
			_ => {}
		}
	}
}

fn count_neighbors(input: &Vec<Vec<Square>>, x: i64, y: i64) -> (usize, usize)
{
	let cords = vec![(-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1)];

	let col_size = input.len();
	let row_size = input[0].len();

	let mut seats: Vec<(i64, i64)> = Vec::new();
	for cord in cords
	{
		match find_seat_in_direction(input, (x, y), cord, (row_size as i64, col_size as i64))
		{
			Some(x) => seats.push(x),
			None => {}
		}
	}
	
	let mut empty = 0;
	let mut taken = 0;
	for seat in seats
	{
		match input[seat.1 as usize][seat.0 as usize] {
			Square::EmptyChair => empty += 1,
			Square::TakenChair => taken += 1,
			Square::Floor => empty +=1,
			_ => {},
		}
	}

	return (empty, taken);

}

fn process_round(input: &Vec<Vec<Square>>) ->  Vec<Vec<Square>>
{
	let mut output: Vec<Vec<Square>> = input.clone();

	for (y, row) in input.iter().enumerate()
	{
		for (x, val) in row.iter().enumerate()
		{
			let count = count_neighbors(input, x as i64, y as i64);

			match val
			{
				Square::EmptyChair => 
				{
					if count.1 == 0
					{
						output[y][x] = Square::TakenChair;
					}
					else
					{
						output[y][x] = Square::EmptyChair;
					}
				}
				Square::TakenChair =>
				{
					if count.1 >= 5
					{
						output[y][x] = Square::EmptyChair;
					}
					else
					{
						output[y][x] = Square::TakenChair;
					}
				}
				_ => {
					output[y][x] = *val;
				}
			}
		}
	}

	return output;
}

fn count_diff(prev_round: &Vec<Vec<Square>>, new_round: &Vec<Vec<Square>>) -> usize
{
	let mut result = 0;
	for col_it in prev_round.iter().zip(new_round.iter())
	{
		let (row1, row2) = col_it;

		for val_it in row1.iter().zip(row2.iter())
		{
			let (val1, val2) = val_it;

			match (val1, val2)
			{
				(Square::EmptyChair, Square::TakenChair) => result += 1,
				(Square::TakenChair, Square::EmptyChair) => result += 1,
				_ => {}
			}

		}
	}

	return result;
}

fn process_until_stable(input: Vec<Vec<Square>>) -> (usize, usize, usize)
{
	let mut prev_round = input;
	let mut rounds = 0;
	let mut empty = 0;
	let mut taken = 0;
	loop
	{
		rounds += 1;
		let new_round = process_round(&prev_round);
		let diff = count_diff(&prev_round, &new_round);
		if diff == 0
		{
			for row in new_round
			{
				for val in row
				{
					match val
					{
						Square::EmptyChair => empty += 1,
						Square::TakenChair => taken += 1,
						_ => {}
					}
				}
			}
			break;
		}
		prev_round = new_round;
	}

	return (rounds, empty, taken);
}

fn main()
{
		let args : Vec<String> = env::args().collect();

		if args.len() < 2
		{
				panic!("Not enough arguments")
		}

		let input_grid = read_file_as_vector(&args[1]);

		let parsed_grid = parse_input(&input_grid);
		let result = process_until_stable(parsed_grid);

		println!("{:?}", result);
}
