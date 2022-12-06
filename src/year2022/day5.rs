use itertools::Itertools;
use std::collections::HashMap;

#[derive(Debug)]
struct Stack {
    items: Vec<char>,
}

impl Stack {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }
}

pub fn solve() {
    let input = include_str!("day5.input.txt");
    let (diagram, moves) = input.split_once("\n\n").unwrap();

    let (stack_names, mut stacks) = read_stacks(diagram);

    for curr_move in moves.lines() {
        let words = curr_move.split_ascii_whitespace().collect::<Vec<&str>>();
        let items = words[1].parse::<u32>().unwrap();
        let from = words[3];
        let to = words[5];

        for _ in 0..items {
            let item = stacks.get_mut(from).unwrap().items.pop().unwrap();
            stacks.get_mut(to).unwrap().items.push(item);
        }
    }

    let top_items = stack_names
        .into_iter()
        .map(|stack_name| stacks.get(stack_name).unwrap().items.last().unwrap())
        .collect::<String>();

    println!("Part 1: {:?}", top_items);

    let (stack_names, mut stacks) = read_stacks(diagram);

    for curr_move in moves.lines() {
        let words = curr_move.split_ascii_whitespace().collect::<Vec<&str>>();
        let items_n = words[1].parse::<u32>().unwrap();
        let from = words[3];
        let to = words[5];

        let mut buf = Vec::<char>::new();

        for _ in 0..items_n {
            let item = stacks.get_mut(from).unwrap().items.pop().unwrap();
            buf.push(item);
        }

        for _ in 0..items_n {
            let item = buf.pop().unwrap();
            stacks.get_mut(to).unwrap().items.push(item);
        }
    }

    let top_items = stack_names
        .into_iter()
        .map(|stack_name| stacks.get(stack_name).unwrap().items.last().unwrap())
        .collect::<String>();

    println!("Part 2: {:?}", top_items);
}

fn read_stacks(input: &str) -> (Vec<&str>, HashMap<&str, Stack>) {
    let mut diagram_iter = input.lines().rev();

    let stack_names = diagram_iter
        .next()
        .unwrap()
        .split("  ")
        .map(|input| input.trim())
        .collect::<Vec<&str>>();

    let mut stacks: HashMap<&str, Stack> = HashMap::new();

    for stack_name in stack_names.iter().rev() {
        stacks.insert(stack_name, Stack::new());
    }

    for row in diagram_iter {
        for (&stack_name, group) in stack_names.iter().zip(row.chars().chunks(4).into_iter()) {
            let stack = stacks.get_mut(stack_name).unwrap();

            let item = group.into_iter().find(|input| input.is_alphabetic());

            match item {
                None => {}
                Some(char) => {
                    stack.items.push(char);
                }
            }
        }
    }

    (stack_names, stacks)
}
