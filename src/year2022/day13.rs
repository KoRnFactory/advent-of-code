use itertools::{EitherOrBoth, Itertools};
use std::cmp::Ordering;
use std::fmt::{Debug, Display};
use std::str::{Chars, FromStr};

#[derive(Debug)]
enum ListItem<T: Debug + Display> {
    Single(T),
    Multiple(Vec<ListItem<T>>),
}

impl<T: Debug + Display + Copy + Ord> PartialEq for ListItem<T> {
    fn eq(&self, other: &Self) -> bool {
        matches!(compare_list_items(self, other), Ordering::Equal)
    }
}

impl<T: Debug + Display + Copy + Ord> ToString for ListItem<T> {
    fn to_string(&self) -> String {
        match self {
            ListItem::Single(x) => x.to_string(),
            ListItem::Multiple(x) => format!("[{}]", x.iter().map(|val| val.to_string()).join(",")),
        }
    }
}

#[derive(Debug)]
struct Pair<T: Debug + Display + Copy> {
    left: ListItem<T>,
    right: ListItem<T>,
}

impl<T: Debug + Display + FromStr + Ord + Copy> Pair<T>
where
    <T as FromStr>::Err: Debug,
{
    fn new(left: ListItem<T>, right: ListItem<T>) -> Self {
        Pair { left, right }
    }

    fn from(input: &str) -> Self {
        let (left_input, right_input) = input.split_once('\n').unwrap();

        Self::new(
            Pair::extract_signal(left_input),
            Pair::extract_signal(right_input),
        )
    }

    fn check_order(&self) -> bool {
        match compare_list_items(&self.left, &self.right) {
            Ordering::Greater => false,
            Ordering::Less => true,
            Ordering::Equal => {
                unreachable!("Sides are equal {:?}", self)
            }
        }
    }

    fn extract_signal(input: &str) -> ListItem<T> {
        extract_list_item(&mut input.chars()).unwrap()
    }
}

pub fn solve() {
    let input = include_str!("day13.input.txt");

    let pairs = input
        .split("\n\n")
        .map(Pair::from)
        .collect::<Vec<Pair<u32>>>();

    let orders = pairs
        .iter()
        .enumerate()
        .filter(|(_, pair)| pair.check_order())
        .collect::<Vec<(usize, &Pair<u32>)>>();

    println!(
        "Part 1: {:?}",
        orders.iter().map(|(index, _)| index + 1).sum::<usize>()
    );

    let mut all_items = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(Pair::extract_signal)
        .collect::<Vec<ListItem<u32>>>();

    let div2 = Pair::<u32>::extract_signal("[[2]]");
    let div6 = Pair::<u32>::extract_signal("[[6]]");

    all_items.push(div2);
    all_items.push(div6);

    all_items.sort_by(|x, y| compare_list_items(x, y));

    println!(
        "Part 2: {:?}",
        all_items
            .iter()
            .enumerate()
            .filter(|(_, packet)| {
                let stringified = packet.to_string();
                stringified == "[[2]]" || stringified == "[[6]]"
            })
            .map(|(index, _)| index + 1)
            .reduce(|acc, x| { acc * x })
            .unwrap()
    );
}

fn extract_list_item<T: FromStr + Debug + Display>(input: &mut Chars) -> Option<ListItem<T>>
where
    <T as FromStr>::Err: Debug,
{
    let mut num = String::from("");
    let mut vecs = vec![vec![]];
    let push_num = |num: String, vecs: &mut Vec<Vec<ListItem<T>>>| {
        let lev = vecs.len() - 1;
        if let Ok(num) = num.parse::<T>() {
            vecs[lev].push(ListItem::Single(num));
        }
        String::from("")
    };
    for c in input {
        match c {
            '[' => vecs.push(vec![]),
            ']' => {
                num = push_num(num, &mut vecs);
                let lev = vecs.len() - 1;
                let v = vecs.pop().unwrap();
                vecs[lev - 1].push(ListItem::Multiple(v));
            }
            ' ' => {}
            ',' => num = push_num(num, &mut vecs),
            d => {
                if d.is_numeric() {
                    num.push(d);
                }
            }
        };
    }

    vecs.pop().map(|v| v.into_iter().next().unwrap())
}

fn compare_list_items<T: Debug + Display + Ord + Copy>(
    left: &ListItem<T>,
    right: &ListItem<T>,
) -> Ordering {
    match left {
        ListItem::Single(l) => match right {
            ListItem::Single(r) => l.cmp(r),
            ListItem::Multiple(r) => {
                let l = vec![ListItem::Single(*l)];
                compare_multiple_list_items(&l, r)
            }
        },
        ListItem::Multiple(l) => match right {
            ListItem::Single(r) => {
                let r = vec![ListItem::Single(*r)];
                compare_multiple_list_items(l, &r)
            }
            ListItem::Multiple(r) => compare_multiple_list_items(l, r),
        },
    }
}

fn compare_multiple_list_items<T: Debug + Display + Ord + Copy>(
    left: &[ListItem<T>],
    right: &[ListItem<T>],
) -> Ordering {
    for pair in left.iter().zip_longest(right.iter()) {
        match pair {
            EitherOrBoth::Left(_) => {
                // println!("Right side finished items");
                return Ordering::Greater;
            }
            EitherOrBoth::Right(_) => {
                return {
                    // println!("Left side finished items");
                    Ordering::Less
                };
            }
            EitherOrBoth::Both(l, r) => {
                // println!("Comparing:\n{:?}\n{:?}\n", l, r);
                match compare_list_items(l, r) {
                    Ordering::Equal => {}
                    x => return x,
                }
            }
        }
    }

    Ordering::Equal
}
