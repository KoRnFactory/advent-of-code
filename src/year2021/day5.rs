use std::cmp::Ordering;
use std::fmt;
use std::fmt::Debug;

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

// #[derive(Debug)]
struct Playground {
    cells: Vec<usize>,
    size: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }
}

impl Line {
    fn new(x1: usize, y1: usize, x2: usize, y2: usize) -> Line {
        Line {
            start: Point::new(x1, y1),
            end: Point::new(x2, y2),
        }
    }

    fn from_points(start: Point, end: Point) -> Line {
        Line { start, end }
    }

    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }
}

impl Playground {
    fn new(dimension: usize) -> Playground {
        let mut cells: Vec<usize> = vec![];
        for _ in 0..dimension.pow(2) {
            cells.push(0);
        }

        Playground {
            cells,
            size: dimension,
        }
    }

    fn mark_line(&mut self, line: Line) {
        let x_trajectory = get_trajectory(line.start.x, line.end.x);
        let y_trajectory = get_trajectory(line.start.y, line.end.y);

        // println!(
        //     "Marking {:?},  trajectory ({}, {})",
        //     line, x_trajectory, y_trajectory
        // );

        let (mut x, mut y) = (line.start.x as i32, line.start.y as i32);

        self.mark_point(x as usize, y as usize);

        let x_end = line.end.x as i32;
        let y_end = line.end.y as i32;

        while x != x_end || y != y_end {
            x += x_trajectory;
            y += y_trajectory;
            self.mark_point(x as usize, y as usize);
        }
    }

    fn mark_point(&mut self, x: usize, y: usize) {
        // println!("Marking point ({}, {})", x, y);
        self.cells[y * self.size + x] += 1;
    }

    fn calculate_score(self) -> u32 {
        self.cells.iter().filter(|&&cell| cell > 1usize).count() as u32
    }
}

impl Debug for Playground {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut str = String::new();
        for i in 0..self.cells.len() {
            str.push_str(&format!("{:?},", &self.cells[i]));
            if i % self.size == self.size - 1 && i != self.cells.len() - 1 {
                str.push_str("\n");
            }
        }
        write!(f, "{}", str)
    }
}

pub fn solve() {
    let input = include_str!("day5.input.txt");

    let lines = input.lines().map(|line| {
        let (start, end) = line.split_once(" -> ").unwrap();
        let (x1, y1) = start.split_once(",").unwrap();
        let (x2, y2) = end.split_once(",").unwrap();
        return Line::new(
            x1.parse::<usize>().unwrap(),
            y1.parse::<usize>().unwrap(),
            x2.parse::<usize>().unwrap(),
            y2.parse::<usize>().unwrap(),
        );
    });

    let mut playground = Playground::new(1000);

    lines
        .clone()
        .filter(|line| line.is_horizontal() || line.is_vertical())
        .for_each(|line| playground.mark_line(line));

    let score = playground.calculate_score();

    println!("Part 1: {:?}", score);

    let mut playground = Playground::new(1000);

    lines.clone().for_each(|line| playground.mark_line(line));

    let score = playground.calculate_score();

    println!("Part 2: {:?}", score);
}

fn get_trajectory(start: usize, end: usize) -> i32 {
    match end.cmp(&start) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}
