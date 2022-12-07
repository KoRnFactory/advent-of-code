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
            let (_, four) = line.split_once(" | ").unwrap();
            let fours = four.split(" ").collect::<Vec<&str>>();

            let meanings = find_all_meanings(line);

            return fours
                .iter()
                .map(|&block| {
                    meanings
                        .iter()
                        .enumerate()
                        .find(|(_, &meaning)| {
                            meaning.len() == block.len()
                                && meaning
                                    .chars()
                                    .all(|meaning_char| block.contains(meaning_char))
                        })
                        .unwrap()
                        .0
                })
                .enumerate()
                .map(|(position, digit)| {
                    digit as u32 * 10u32.pow((fours.len() as u32 - 1u32) - position as u32)
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

fn find_all_meanings(line: &str) -> Vec<&str> {
    let (ten, four) = line.split_once(" | ").unwrap();
    let tens = ten.split(" ").collect::<Vec<&str>>();
    let fours = four.split(" ").collect::<Vec<&str>>();

    let all_blocks = tens
        .iter()
        .chain(fours.iter())
        .map(|&number| number)
        .collect::<Vec<&str>>();

    let mut meanings = vec![""; 10];

    meanings[1] = all_blocks.iter().find(|block| block.len() == 2).unwrap();
    meanings[4] = all_blocks.iter().find(|block| block.len() == 4).unwrap();
    meanings[7] = all_blocks.iter().find(|block| block.len() == 3).unwrap();
    meanings[8] = all_blocks.iter().find(|block| block.len() == 7).unwrap();

    let fourdiff = meanings[4]
        .chars()
        .filter(|&char| !(meanings[7].contains(char)))
        .collect::<Vec<char>>();

    let five_lenghts = all_blocks
        .iter()
        .filter(|&&meaning| meaning.len() == 5)
        .map(|&meaning| {
            let index: usize = if meanings[1].chars().all(|char| meaning.contains(char)) {
                3
            } else if fourdiff.iter().all(|&char| meaning.contains(char)) {
                5
            } else {
                2
            };

            (index, meaning)
        })
        .collect::<Vec<(usize, &str)>>();

    for (index, meaning) in five_lenghts {
        meanings[index] = meaning
    }

    let six_lenghts = all_blocks
        .iter()
        .filter(|&&meaning| meaning.len() == 6)
        .map(|meaning| {
            let index: usize = if meanings[4].chars().all(|char| meaning.contains(char)) {
                9
            } else if fourdiff.iter().all(|&char| meaning.contains(char)) {
                6
            } else {
                0
            };

            (index, *meaning)
        })
        .collect::<Vec<(usize, &str)>>();

    for (index, meaning) in six_lenghts {
        meanings[index] = meaning
    }

    return meanings;
}
