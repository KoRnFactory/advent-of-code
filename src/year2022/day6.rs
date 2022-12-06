use itertools::Itertools;

pub fn solve() {
    let input = include_str!("day6.input.txt");

    println!("Part 1: {:?}", last_unique_n(input, 4));
    println!("Part 2: {:?}", last_unique_n(input, 14));
}

fn last_unique_n(input: &str, n: usize) -> usize {
    let mut iter = input.chars().enumerate();

    let mut last_n = Vec::<char>::new();
    loop {
        let (index, packet) = iter.next().unwrap();

        last_n.push(packet);

        if last_n.len() > n {
            last_n.remove(0);

            if last_n.iter().all_unique() {
                break index;
            }
        }
    }
}
