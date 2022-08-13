use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Cell {
	value: u32,
	marked: bool,
}

impl Cell {
	fn new(value: u32) -> Cell {
		Cell {
			value,
			marked: false,
		}
	}

	fn mark(&mut self) {
		self.marked = true;
	}
}

#[derive(Debug)]
struct Board {
	cells: Vec<Vec<Cell>>,
	last_called: Option<u32>,
}


impl Board {
	fn new(input: &Vec<Vec<String>>) -> Board {
		let cells = input.iter().map(|row| {
			row.iter().map(|cell| {
				Cell::new(cell.parse::<u32>().unwrap())
			}).collect()
		}).collect();

		Board {
			cells,
			last_called: None,
		}
	}

	fn mark(&mut self, number: u32) {
		let mut has_been_marked = false;
		for row in self.cells.iter_mut() {
			for cell in row.iter_mut() {
				if cell.value == number {
					has_been_marked = true;
					cell.mark()
				}
			}
		}

		if has_been_marked {
			self.last_called = Some(number);
		}
	}

	fn has_won(&self) -> bool {
		for row in self.cells.iter() {
			if row.iter().all(|cell| cell.marked) {
				return true;
			}
		}

		for col in 0..self.cells[0].len() {
			if self.cells.iter().map(|row| row[col].marked).all(|marked| marked) {
				return true;
			}
		}

		return false;
	}

	fn winning_steps(&mut self, numbers: &Vec<u32>) -> Option<usize> {
		let mut steps: usize = 0;
		for current in numbers {
			steps += 1;
			self.mark(*current);
			if self.has_won() {
				break;
			}
		};

		return if steps < numbers.len() { Some(steps) } else { None };
	}

	fn calculate_score(&self) -> u32 {
		let board_portion: u32 = self.cells.iter().flatten().filter(|cell| cell.marked).map(|cell| cell.value).sum();
		board_portion * self.last_called.unwrap()
	}
}

pub(crate) fn solve() {
	let file = File::open("./src/day4/dummy.txt").unwrap();
	let buffer = BufReader::new(file);
	let lines = buffer
		.lines()
		.map(|l| l.expect("Could not parse line"))
		.collect::<Vec<String>>();

	let mut lines_iter = lines.iter();

	let generated_numbers = lines_iter.next().unwrap().split(",").map(|n| n.parse::<u32>().unwrap()).collect::<Vec<u32>>();
	lines_iter.next();

	let mut boards: Vec<Board> = Vec::new();

	let mut board_input: Vec<Vec<String>> = Vec::new();

	for line in lines_iter {
		if line.is_empty() {
			boards.push(Board::new(&board_input));
			board_input.clear();
		} else {
			let row = line.split(" ").map(|x| String::from(x)).filter(|x| !x.is_empty()).collect();
			board_input.push(row);
		}
	}
	boards.push(Board::new(&board_input));

	for board in boards {
		println!("{:?}", board);
	}
}

fn calculate_earliest_winning_board(boards: &Vec<Board>, numbers: &Vec<u32>) -> Option<Board> {
	let winning_steps: Vec<Option<usize>> = boards.iter().map(|mut board| board.winning_steps(numbers)).collect();

	let best_board_index = winning_steps.iter().fold(None, |acc: Option<usize>, current| if current.is_some() {
		match acc {
			Some(best) => {
				Some(current.unwrap().min(best))
			}
			None => *current
		}
	} else { acc });
}


