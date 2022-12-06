use itertools::Itertools;

const BASE: &str = "abcdefg";

pub fn solve() {
    let input = include_str!("day8.input.txt");

    println!("Part 1: {:?}", part1(input));
    println!("Part 2: {:?}", part2(input));
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (_, four) = line.split_once(" | ").unwrap();
            four.split(" ")
                .filter(|block| match get_sure_value(block) {
                    Some(_) => true,
                    None => false,
                })
                .count()
        })
        .sum::<usize>() as u32
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let (ten, four) = line.split_once(" | ").unwrap();
            let tens = ten.split(" ").collect::<Vec<&str>>();
            let fours = four.split(" ").collect::<Vec<&str>>();

            let all_blocks = tens
                .iter()
                .chain(fours.iter())
                .map(|&number| number)
                .collect::<Vec<&str>>();

            let seed = find_seed(&all_blocks).unwrap();

            let meanings = fours
                .iter()
                .map(|&number| find_meaning(number, &seed).unwrap())
                .collect::<Vec<u32>>();

            return meanings
                .iter()
                .enumerate()
                .map(|(position, digit)| {
                    digit * 10u32.pow((meanings.len() as u32 - 1u32) - position as u32)
                })
                .sum::<u32>();
        })
        .sum::<u32>()
}

fn get_sure_value(input: &str) -> Option<&str> {
    match input.len() {
        2 => Some("1"),
        4 => Some("4"),
        3 => Some("7"),
        7 => Some("8"),
        _ => None,
    }
}

fn find_seed(numbers: &Vec<&str>) -> Option<String> {
    for seed_vec in BASE.chars().permutations(BASE.len()).unique() {
        let seed = seed_vec.iter().collect::<String>();

        if numbers
            .iter()
            .all(|&number| find_meaning(number, &seed).is_some())
        {
            return Some(seed.to_string());
        }
    }

    return None;
}

fn generate_seed(seed: &String) -> Vec<String> {
    (0..10 as usize)
        .map(|x| generate_number(x, seed.as_str()).to_owned())
        .collect::<Vec<String>>()
}

fn generate_number(number: usize, seed: &str) -> String {
    let seed_chars = seed.chars().collect::<Vec<char>>();

    let mut numbers: Vec<char> = vec![];

    let positions: Vec<usize> = match number {
        0 => vec![0, 1, 2, 4, 5, 6],
        1 => vec![2, 5],
        2 => vec![0, 2, 3, 4, 6],
        3 => vec![0, 2, 3, 5, 6],
        4 => vec![1, 2, 3, 5],
        5 => vec![0, 1, 3, 5, 6],
        6 => vec![0, 1, 3, 4, 5, 6],
        7 => vec![0, 2, 5],
        8 => vec![0, 1, 2, 3, 4, 5, 6],
        9 => vec![0, 1, 2, 3, 5, 6],
        _ => vec![],
    };

    for x in positions {
        numbers.push(seed_chars[x]);
    }

    return numbers.iter().collect();
}

fn find_meaning(search: &str, seed: &str) -> Option<u32> {
    let seed_numbers = generate_seed(&seed.to_string());

    let item = seed_numbers
        .iter()
        .map(|number| number.as_str())
        .enumerate()
        .filter(|(_, number)| number.len() == search.len())
        .find(|(_, seed_number)| {
            let result = seed_number
                .chars()
                .all(|seed_char| search.contains(seed_char))
                && search
                    .chars()
                    .all(|search_char| seed_number.contains(search_char));

            return result;
        });

    match item {
        None => None,
        Some((index, _)) => Some(index as u32),
    }
}
