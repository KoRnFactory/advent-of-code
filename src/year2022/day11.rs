use num::integer::lcm;
use std::fmt::{Debug, Formatter};
use std::ops::{Add, Div, Mul, Sub};

type Item = u128;
type Operation = Box<dyn Fn(Item) -> Item>;
type Actions = (usize, usize);

struct Monkey {
    items: Vec<Item>,
    operation: Operation,
    actions: Actions,
    inspected_items: usize,
    test_number: Item,
}

impl Monkey {
    fn new(items: Vec<Item>, operation: Operation, test_number: Item, actions: Actions) -> Self {
        Self {
            items,
            operation,
            test_number,
            actions,
            inspected_items: 0,
        }
    }

    fn inspect(&mut self) -> Option<Item> {
        if self.items.is_empty() {
            return None;
        }

        self.inspected_items += 1;
        Some(self.items.remove(0))
    }

    fn receive(&mut self, item: Item) {
        self.items.push(item);
    }

    fn test(&self, number: Item) -> bool {
        number % self.test_number == 0
    }
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\nMonkey:\nItems: {:?}\nThrow to: {:?}\nInspected items: {:?}",
            self.items, self.actions, self.inspected_items
        )
    }
}

pub fn solve() {
    let input = include_str!("day11.input.txt");

    let mut monkeys = get_monkeys(input);

    let lcm_monkeys = get_monkeys_lcm(&monkeys);

    for _ in 0..20 {
        for index in 0..monkeys.len() {
            loop {
                let curr_item = monkeys[index].inspect();

                if let Some(old_level) = curr_item {
                    let level = (monkeys[index].operation)(old_level) as Item;

                    let mut level = level / 3 as Item;

                    level = level % lcm_monkeys;

                    let recipient = if monkeys[index].test(level) {
                        monkeys[index].actions.1
                    } else {
                        monkeys[index].actions.0
                    };

                    monkeys[recipient].receive(level);
                } else {
                    break;
                }
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspected_items.cmp(&a.inspected_items));

    println!(
        "Part 1: {:?}",
        monkeys.get(0).unwrap().inspected_items * monkeys.get(1).unwrap().inspected_items
    );

    let mut monkeys = get_monkeys(input);

    let lcm_monkeys = get_monkeys_lcm(&monkeys);

    for _ in 0..10000 {
        for index in 0..monkeys.len() {
            loop {
                let curr_item = monkeys[index].inspect();

                if let Some(old_level) = curr_item {
                    let mut level = (monkeys[index].operation)(old_level) as Item;

                    level = level % lcm_monkeys;

                    let recipient = if monkeys[index].test(level) {
                        monkeys[index].actions.1
                    } else {
                        monkeys[index].actions.0
                    };

                    monkeys[recipient].receive(level);
                } else {
                    break;
                }
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspected_items.cmp(&a.inspected_items));

    println!(
        "Part 2: {:?}",
        monkeys.get(0).unwrap().inspected_items * monkeys.get(1).unwrap().inspected_items
    );
}

fn get_monkeys(input: &'static str) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = vec![];

    for monkey in input.split("\n\n") {
        let mut items: Vec<Item> = vec![];
        let mut operation: Operation = Box::new(|item| item);
        let mut test: Item = 0;
        let mut actions: Actions = (0, 0);

        for line in monkey.lines() {
            let tokens = line.trim().split_once(": ");

            if let Some((start, end)) = tokens {
                match start {
                    "Starting items" => {
                        for item in end.split(", ") {
                            items.push(item.parse::<Item>().expect("Malformed items"))
                        }
                    }
                    "Operation" => {
                        let (_, op) = end.split_once(" = ").expect("Malformed operation");
                        let (left, rest) = op.split_once(" ").expect("Malformed operation");
                        let (sign, right) = rest.split_once(" ").expect("Malformed operation");

                        operation = Box::new(move |old| {
                            let left = left.parse::<Item>().unwrap_or(old);
                            let right = right.parse::<Item>().unwrap_or(old);

                            match sign {
                                "+" => left.add(right),
                                "-" => left.sub(right),
                                "*" => left.mul(right),
                                "/" => left.div(right),
                                _ => {
                                    unreachable!()
                                }
                            }
                        })
                    }
                    "Test" => test = end.split(" ").last().unwrap().parse::<Item>().unwrap(),
                    "If true" => {
                        actions.1 = end
                            .split(" ")
                            .last()
                            .unwrap()
                            .parse::<usize>()
                            .expect("Malformed true statement")
                    }
                    "If false" => {
                        actions.0 = end
                            .split(" ")
                            .last()
                            .unwrap()
                            .parse::<usize>()
                            .expect("Malformed false statement")
                    }
                    &_ => {
                        unreachable!("What's this")
                    }
                }
            }
        }

        monkeys.push(Monkey::new(items, operation, test, actions))
    }

    return monkeys;
}

fn get_monkeys_lcm(monkeys: &Vec<Monkey>) -> Item {
    monkeys
        .iter()
        .map(|monkey| monkey.test_number)
        .reduce(|acc, curr| lcm(acc, curr))
        .unwrap()
}
