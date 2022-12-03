pub fn solve() {
    let input = include_str!("day3.input.txt");

    let sacks = input.lines();

    let priorities_1 = sacks
        .clone()
        .flat_map(|sack| find_repeated_item(sack))
        .flat_map(|item| calculate_priority(item));

    println!("Part 1: {:?}", priorities_1.sum::<u32>());

    let mut index_within_group = 0;

    let priorities_2 = input
        .clone()
        .split(|char: char| {
            if !(char.is_whitespace()) {
                return false;
            }

            return if index_within_group == 2 {
                index_within_group = 0;
                true
            } else {
                index_within_group += 1;
                false
            };
        })
        .flat_map(|sack| find_badge_type(sack))
        .flat_map(|item| calculate_priority(item));

    println!("Part 2: {:?}", priorities_2.sum::<u32>());
}

fn calculate_priority(item: char) -> Option<u32> {
    let ascii = Some(item as u32);

    return match ascii {
        None => None,
        Some(ascii) => {
            let factor: u32 = if item.is_ascii_lowercase() {
                97 - 1
            } else {
                65 - 27
            };

            Some(ascii - factor)
        }
    };
}

fn find_repeated_item(input: &str) -> Option<char> {
    let (left, right) = input.split_at((input.len()) / 2);
    return right.chars().find(|right_char| {
        left.chars()
            .find(|left_char| left_char.eq(right_char))
            .is_some()
    });
}

fn find_badge_type(input: &str) -> Option<char> {
    let elves = input.lines().collect::<Vec<&str>>();

    if elves.len() < 3 {
        return None;
    };

    let mut possibilities = elves[0].chars().filter(|first_char| {
        elves[1]
            .chars()
            .find(|second_char| second_char.eq(first_char))
            .is_some()
    });

    return possibilities.find(|possibility| {
        elves[2]
            .chars()
            .find(|third_char| third_char.eq(possibility))
            .is_some()
    });
}
