use std::fs::File;
use std::io;
use std::io::BufRead;

pub fn solve() {
    by_window();
}

fn by_line() {
    let file = File::open("./src/day1/input.txt").unwrap();
    let buffer = io::BufReader::new(file);
    let lines = buffer.lines();

    let mut count = 0;
    let mut previous: Option<i32> = None;

    lines.for_each(|line| {
        let current = line.unwrap().parse::<i32>().unwrap();
        if previous.is_some() {
            let previous_value = previous.unwrap();
            if current > previous_value {
                count += 1;
            }
        }
        previous = Some(current);
    });

    println!("{}", count);
}

fn by_window() {
    let file = File::open("./src/aoc/day1/input.txt").unwrap();
    let buffer = io::BufReader::new(file);
    let lines = buffer.lines();

    let mut count = 0;
    let mut window: Vec<i32> = Vec::new();

    lines.for_each(|line| {
        let current = line.unwrap().parse::<i32>().unwrap();
        if window.len() == 3 {
            let previous_measurement = window.iter().sum::<i32>();
            let current_measurement = previous_measurement - window[0] + current;
            if current_measurement > previous_measurement {
                count += 1;
            }
            window.remove(0);
        }
        window.push(current);
    });

    println!("{}", count);
}
