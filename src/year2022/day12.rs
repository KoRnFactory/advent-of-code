use std::fmt::Debug;
use std::ops::Add;

const STARTING_CHAR: char = 'S';
const ENDING_CHAR: char = 'E';

type Position = (usize, usize);

#[derive(Debug, Clone, Copy)]
enum Visit<T> {
    Visited(T),
    Visiting(T),
    Unvisited,
}

impl<T> Default for Visit<T> {
    fn default() -> Self {
        Visit::Unvisited
    }
}

pub fn solve() {
    let input = include_str!("day12.input.txt");

    let grid = get_grid(input);
    let starting_pos = get_positions(&grid, STARTING_CHAR)[0];
    let ending_pos = get_positions(&grid, ENDING_CHAR)[0];

    let visits_map_from_start = calculate_shortest_path(&grid, ending_pos);
    let shortest_paths_from_start = convert_visits_map(&visits_map_from_start);

    let shortest_path_from_start =
        shortest_paths_from_start[starting_pos.1][starting_pos.0].unwrap();

    println!("Part 1: {:?}", shortest_path_from_start);

    let starting_positions = get_positions(&grid, 'a');

    let shortest_paths_from_lowest = starting_positions
        .iter()
        .map(|&starting_pos| shortest_paths_from_start[starting_pos.1][starting_pos.0])
        .flatten()
        .min();

    println!(
        "Part 2: {:?}",
        shortest_paths_from_lowest
            .map(|min| min.min(shortest_path_from_start))
            .unwrap()
    );
}

fn get_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect()
}

fn get_positions(grid: &Vec<Vec<char>>, search: char) -> Vec<Position> {
    let row_length = grid[0].len();

    grid.iter()
        .fold(vec![], |mut acc, row| {
            row.iter().for_each(|visit| acc.push(visit));
            return acc;
        })
        .iter()
        .enumerate()
        .filter(|(_, &&char)| char == search)
        .map(|(index, _)| (index % row_length, index / row_length))
        .collect::<Vec<Position>>()
}

fn get_height(char: char) -> u32 {
    match char {
        STARTING_CHAR => 0,
        ENDING_CHAR => 26,
        value => value.to_digit(36).unwrap() - 10,
    }
}

fn can_move(grid: &Vec<Vec<char>>, from: Position, to: Position) -> bool {
    let from_height = get_height(grid[from.1][from.0]);
    let to_height = get_height(grid[to.1][to.0]);

    return !(to_height as i32 - from_height as i32 > 1);
}

fn is_in_bounds(grid: &Vec<Vec<char>>, candidate: (i32, i32)) -> bool {
    return !(candidate.0 < 0
        || candidate.1 < 0
        || candidate.0 >= grid[0].len() as i32
        || candidate.1 >= grid.len() as i32);
}

fn print_visits<T: Debug>(grid: &Vec<Vec<T>>) {
    for line in grid {
        for value in line {
            print!("{:?} ", value)
        }
        print!("\n");
    }
    print!("\n");
}

fn calculate_shortest_path(grid: &Vec<Vec<char>>, ending_pos: Position) -> Vec<Vec<Visit<u32>>> {
    let mut visits = vec![vec![Visit::Unvisited; grid[0].len()]; grid.len()];

    visits[ending_pos.1][ending_pos.0] = Visit::Visiting(0);

    while let Some(visiting) = find_next_visit(&visits) {
        visits[visiting.1][visiting.0] = match visits[visiting.1][visiting.0] {
            Visit::Visiting(current) => Visit::Visited(current),
            _ => {
                print_visits(&visits);
                unreachable!(
                    "Entered already visited or completely unvisited node: {:?}, that is: {:?}",
                    visiting, visits[visiting.1][visiting.0]
                )
            }
        };

        for delta in vec![(-1, 0), (0, -1), (1, 0), (0, 1)] {
            let candidate = (visiting.0 as i32 + delta.0, visiting.1 as i32 + delta.1);

            if !is_in_bounds(grid, candidate) {
                continue;
            }

            let candidate: Position = (
                delta.0.add(visiting.0 as i32) as usize,
                delta.1.add(visiting.1 as i32) as usize,
            );

            if can_move(grid, candidate, visiting) {
                let next_step = match visits[visiting.1][visiting.0] {
                    Visit::Visited(steps) => steps,
                    _ => unreachable!(),
                } + 1;
                match visits[candidate.1][candidate.0] {
                    Visit::Visited(_) => {}
                    Visit::Visiting(x) => {
                        if next_step < x {
                            visits[candidate.1][candidate.0] = Visit::Visiting(next_step)
                        }
                    }
                    Visit::Unvisited => {
                        visits[candidate.1][candidate.0] = Visit::Visiting(next_step)
                    }
                }
            }
        }
    }

    return visits;
}

fn find_next_visit(visits: &Vec<Vec<Visit<u32>>>) -> Option<Position> {
    let row_length = visits[0].len();

    return visits
        .iter()
        .fold(vec![], |mut acc, row| {
            row.iter().for_each(|visit| acc.push(visit));
            return acc;
        })
        .iter()
        .enumerate()
        .filter(|(_, visit)| match visit {
            Visit::Visiting(_) => true,
            _ => false,
        })
        .reduce(|best, curr| {
            return match curr.1 {
                Visit::Visited(_) => best,
                Visit::Visiting(curr_length) => match best.1 {
                    Visit::Visiting(best_length) => {
                        if curr_length < best_length {
                            curr
                        } else {
                            best
                        }
                    }
                    _ => curr,
                },
                Visit::Unvisited => best,
            };
        })
        .map(|(index, _)| (index % row_length, index / row_length));
}

fn convert_visits_map<T: Clone + Copy>(visits: &Vec<Vec<Visit<T>>>) -> Vec<Vec<Option<T>>> {
    let mut map = vec![vec![None; visits[0].len()]; visits.len()];

    for row in 0..visits.len() {
        for col in 0..visits[0].len() {
            map[row][col] = match visits[row][col] {
                Visit::Visited(x) => Some(x),
                _ => None,
            }
        }
    }

    map
}
