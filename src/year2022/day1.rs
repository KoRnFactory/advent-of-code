pub fn solve() {
    let lines = include_str!("day1.input.txt");

    let elves = lines.split("\n\n");

    let mut best_elves: Vec<u32> = elves.map(|line| line.split("\n").flat_map(|cals| cals.parse::<u32>()).sum()).collect();

    best_elves.sort_by(|a, b| b.cmp(a));

    println!("Part 1: {:?}", best_elves[0]);
    println!("Part 2: {:?}", best_elves.iter().take(3).sum::<u32>());
}
