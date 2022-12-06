const FIRST_CYCLE: usize = 9;
const CYCLE: usize = 7;

pub fn solve() {
    let input = include_str!("day6.input.txt");

    let school = input
        .lines()
        .next()
        .unwrap()
        .split(",")
        .flat_map(|fish| fish.parse::<u32>())
        .collect::<Vec<u32>>();

    println!("Part 1: {:?}", after_n_generations(&school, 80));
    println!("Part 2: {:?}", after_n_generations(&school, 256));
}

fn after_n_generations(input: &Vec<u32>, n: u32) -> u64 {
    let mut school: Vec<u64> = Vec::from([0, 0, 0, 0, 0, 0, 0, 0, 0]);

    for &x in input {
        school[x as usize] += 1;
    }

    for _ in 0..n {
        let new_borns = school.remove(0);
        school.push(new_borns);
        school[CYCLE - 1] += school[FIRST_CYCLE - 1];
    }

    return school.iter().sum();
}
