use std::fs::File;
use std::io::{BufRead, BufReader};

struct Pos {
	horizontal: usize,
	depth: usize,
	aim: usize,
}

impl Pos {
	fn new() -> Pos {
		Pos {
			horizontal: 0,
			depth: 0,
			aim: 0,
		}
	}

	fn move_by(&mut self, direction: &str, count: usize) {
		match direction {
			"forward" => {
				self.horizontal += count;
				self.depth += count * self.aim
			}
			"down" => self.aim += count,
			"up" => self.aim -= count,
			_ => panic!("Invalid direction"),
		}
	}

	fn calculate_result(&self) -> usize {
		self.horizontal * self.depth
	}
}

pub(crate) fn solve() {
	let file = File::open("./src/day2/input.txt").unwrap();
	let buffer = BufReader::new(file);
	let lines = buffer.lines();

	let mut position = Pos::new();

	lines.for_each(|line| {
		let current = line.unwrap().parse::<String>().unwrap();
		let instruction: Vec<&str> = current.split_whitespace().collect();

		let (direction, distance) = (instruction[0], instruction[1].parse::<usize>().unwrap());

		position.move_by(direction, distance);
	});

	println!("{}", position.calculate_result());
}
