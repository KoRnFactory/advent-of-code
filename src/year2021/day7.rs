pub fn solve() {
    let input = include_str!("day7.input.txt");

    let crabs = input
        .lines()
        .next()
        .unwrap()
        .split(",")
        .flat_map(|fish| fish.parse::<u32>())
        .collect::<Vec<u32>>();

    let &max_position = crabs.iter().max().unwrap();

    let best_part_1 = (0..max_position)
        .map(|position| calculate_fuel_cons_part1(&crabs, position))
        .min()
        .unwrap();

    println!("Part1: {:?}", best_part_1);

    let best_part_2 = (0..max_position)
        .map(|position| calculate_fuel_cons_part2(&crabs, position))
        .min()
        .unwrap();

    println!("Part2: {:?}", best_part_2);
}

fn calculate_fuel_cons_part1(crabs: &Vec<u32>, position: u32) -> u32 {
    return crabs.iter().map(|&crab| crab.abs_diff(position)).sum();
}

fn calculate_fuel_cons_part2(crabs: &Vec<u32>, position: u32) -> u32 {
    return crabs
        .iter()
        .map(|&crab| calculate_fuel_cons_part2_single(crab, position))
        .sum();
}

fn calculate_fuel_cons_part2_single(from: u32, to: u32) -> u32 {
    let steps = from.abs_diff(to);

    let base_result = (steps + 1) * (steps / 2);

    match steps % 2 {
        1 => base_result + (steps + 1) / 2,
        _ => base_result,
    }
}
