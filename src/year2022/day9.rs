use std::collections::HashSet;

pub fn solve() {
    let input = include_str!("day9.input.txt");

    println!("Part1: {:?}", get_tail_positions(input, 2).iter().count());
    println!("Part2: {:?}", get_tail_positions(input, 10).iter().count());
}

fn get_tail_positions(input: &str, snake_size: usize) -> HashSet<String> {
    let mut set = HashSet::<String>::new();

    let mut snake = vec![(0i32, 0i32); snake_size];

    set.insert(hash_position(*snake.last().unwrap()));

    for line in input.lines() {
        let (direction, amount) = line
            .split_once(" ")
            .map(|(dir, am)| (dir, am.parse::<u32>().expect("Malformed input")))
            .unwrap();

        for _ in 0..amount {
            for body in 0..snake.iter().len() {
                if body == 0 {
                    match direction {
                        "U" => snake[body].1 += 1,
                        "D" => snake[body].1 -= 1,
                        "L" => snake[body].0 -= 1,
                        "R" => snake[body].0 += 1,
                        _ => {}
                    }
                } else {
                    let x_abs_diff = snake[body - 1].0.abs_diff(snake[body].0) as i32;
                    let y_abs_diff = snake[body - 1].1.abs_diff(snake[body].1) as i32;

                    if x_abs_diff + y_abs_diff > 1 {
                        let x_dist = snake[body - 1].0 - snake[body].0;
                        let y_dist = snake[body - 1].1 - snake[body].1;

                        let x_move = if x_abs_diff > 1 || x_abs_diff + y_abs_diff > 2 {
                            Some(x_dist / x_abs_diff)
                        } else {
                            None
                        };

                        let y_move = if y_abs_diff > 1 || x_abs_diff + y_abs_diff > 2 {
                            Some(y_dist / y_abs_diff)
                        } else {
                            None
                        };

                        if x_move.is_some() {
                            snake[body].0 += x_move.unwrap();
                        }

                        if y_move.is_some() {
                            snake[body].1 += y_move.unwrap();
                        }

                        if body == snake.len() - 1 {
                            set.insert(hash_position(snake[body]));
                        }
                    }
                }
            }
        }
    }

    return set;
}

fn hash_position(pos: (i32, i32)) -> String {
    let (x, y) = pos;

    let mut hash = "".to_string();
    hash.push_str(x.to_string().as_str());
    hash.push_str(",".to_string().as_str());
    hash.push_str(y.to_string().as_str());

    return hash;
}
