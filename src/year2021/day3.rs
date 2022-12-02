use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve() {
    let file = File::open("./src/year2021/day3.input.txt").unwrap();
    let buffer = BufReader::new(file);
    let lines = buffer
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect::<Vec<String>>();

    let power = get_life_support_rating(lines);

    println!("{}", power);
}

fn get_power(report: Vec<String>) -> u64 {
    let mut gamma_str = String::new();
    let mut epsilon_str = String::new();

    let high_occurrences = get_high_occurrences(&report);

    for high_count in high_occurrences {
        let low_count = report.len() - high_count;

        if high_count > low_count {
            gamma_str.push('1');
            epsilon_str.push('0');
        } else {
            epsilon_str.push('1');
            gamma_str.push('0');
        }
    }

    let gamma = binary_string_to_number(&gamma_str);
    let epsilon = binary_string_to_number(&epsilon_str);

    gamma * epsilon
}

fn get_life_support_rating(report: Vec<String>) -> u64 {
    let oxygen_generator_rating =
        find_rating_by_criteria(
            &report,
            |high_count, low_count| if low_count > high_count { '0' } else { '1' },
        );

    let co2_scrubber_rating =
        find_rating_by_criteria(
            &report,
            |high_count, low_count| if low_count > high_count { '1' } else { '0' },
        );

    co2_scrubber_rating * oxygen_generator_rating
}

fn find_rating_by_criteria(report: &Vec<String>, criteria: fn(usize, usize) -> char) -> u64 {
    let mut current_values = report.clone();

    for bit in 0..report[0].len() {
        let high_count = get_position_high_count(&current_values, bit);
        let low_count = current_values.len() - high_count;

        let desired_value = criteria(high_count, low_count);

        current_values = current_values
            .into_iter()
            .filter(|x| x.chars().nth(bit).unwrap() == desired_value)
            .collect();

        if current_values.len() == 1 {
            break;
        }
    }

    binary_string_to_number(&current_values[0])
}

fn binary_string_to_number(s: &str) -> u64 {
    s.chars()
        .map(|c| c.to_digit(2).unwrap())
        .fold(0, |acc, d| acc * 2 + d as u64)
}

fn get_high_occurrences(report: &Vec<String>) -> Vec<usize> {
    let bits = report[0].len();
    let mut high_occurrences = vec![0; bits];
    for i in 0..bits {
        let bit_count = get_position_high_count(report, i);
        high_occurrences[i] = bit_count;
    }
    high_occurrences
}

fn get_position_high_count(report: &Vec<String>, position: usize) -> usize {
    let mut bit_count: usize = 0;
    for line in report.iter() {
        if line.chars().nth(position).unwrap() == '1' {
            bit_count += 1;
        }
    }
    bit_count
}
