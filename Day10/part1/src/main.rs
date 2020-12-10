use std::env;
use std::fs::File;
use std::io::{self, BufRead};

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

fn parse_input(input: &Vec<String>) -> Vec<i64>
{
	let result: Vec<i64> = input.iter().map(|x| x.parse().unwrap()).collect();
	return result;
}


fn calculate_result(input: &mut Vec<i64>) -> (i64, i64)
{
	let mut result : (i64, i64) = (0,0);

	let mut calc_vec = vec![0];
	calc_vec.append(input);

	for p in calc_vec.windows(2)
	{
		let tmp = p[1] - p[0];
		match tmp
		{
			1 => result.0 +=1,
			3 => result.1 += 1,
			_ => panic!("invalid diff")
		};
	};

	result.1 += 1;
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
	let mut parsed_input = parse_input(&input);
	parsed_input.sort();

	let result = calculate_result(&mut parsed_input);
	println!("{:?}", result.0 * result.1);
}
