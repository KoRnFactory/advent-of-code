use std::fmt::{Debug, Formatter};
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

struct Board {
    cells: Vec<Vec<Cell>>,
    last_called: Option<u32>,
    winning_steps: Option<usize>,
}

impl Board {
    fn new(input: &Vec<Vec<String>>) -> Board {
        let cells = input
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| Cell::new(cell.parse::<u32>().unwrap()))
                    .collect()
            })
            .collect();

        Board {
            cells,
            last_called: None,
            winning_steps: None,
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
            if self
                .cells
                .iter()
                .map(|row| row[col].marked)
                .all(|marked| marked)
            {
                return true;
            }
        }

        return false;
    }

    fn simulate(&mut self, numbers: &Vec<u32>) -> () {
        let mut steps: usize = 0;
        for current in numbers {
            steps += 1;
            self.mark(*current);
            if self.has_won() {
                break;
            }
        }

        self.winning_steps = if steps < numbers.len() {
            Some(steps)
        } else {
            None
        };
    }

    fn calculate_score(&self) -> u32 {
        let board_portion: u32 = self
            .cells
            .iter()
            .flatten()
            .filter(|cell| cell.marked == false)
            .map(|cell| cell.value)
            .sum();

        board_portion * self.last_called.unwrap()
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Board:\n{:?}\nlast called: {:?}",
            self.cells, self.last_called
        )
    }
}

pub fn solve() {
    let file = File::open("./src/year2021/day4.input.txt").unwrap();
    let buffer = BufReader::new(file);
    let lines = buffer
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect::<Vec<String>>();

    let mut lines_iter = lines.iter();

    let generated_numbers = lines_iter
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    lines_iter.next();

    let mut boards: Vec<Board> = Vec::new();

    let mut board_input: Vec<Vec<String>> = Vec::new();

    for line in lines_iter {
        if line.is_empty() {
            let mut board = Board::new(&board_input);
            board.simulate(&generated_numbers);
            boards.push(board);
            board_input.clear();
        } else {
            let row = line
                .split(" ")
                .map(|x| String::from(x))
                .filter(|x| !x.is_empty())
                .collect();
            board_input.push(row);
        }
    }
    let mut board = Board::new(&board_input);
    board.simulate(&generated_numbers);
    boards.push(board);

    boards.sort_by(|a, b| a.winning_steps.cmp(&b.winning_steps));

    let best_board = boards.iter().find(|board| board.winning_steps.is_some());

    let best_result = match best_board {
        None => 0,
        Some(board) => board.calculate_score(),
    };

    println!("Best result: {:?}", best_result);

    let worst_board = boards.iter().rfind(|board| board.winning_steps.is_some());

    let worst_result = match worst_board {
        None => 0,
        Some(board) => board.calculate_score(),
    };

    println!("Worst result: {:?}", worst_result);
}
