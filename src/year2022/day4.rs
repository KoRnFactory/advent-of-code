use std::cmp::max;

#[derive(Debug)]
struct Job {
    start_zone: usize,
    end_zone: usize,
}

impl Job {
    fn new(start_zone: usize, end_zone: usize) -> Self {
        Job {
            start_zone,
            end_zone,
        }
    }
}

pub fn solve() {
    let input = include_str!("day4.input.txt");
    let lines = input.lines();

    let mut last_zone: usize = 0;

    let pairs = lines
        .map(|line| {
            let (first_elf, second_elf) = line.split_once(",").unwrap();

            let (fs, fe) = first_elf.split_once("-").unwrap();
            let (ss, se) = second_elf.split_once("-").unwrap();

            let first_start_zone = fs.parse::<usize>().unwrap();
            let first_end_zone = fe.parse::<usize>().unwrap();
            let second_start_zone = ss.parse::<usize>().unwrap();
            let second_end_zone = se.parse::<usize>().unwrap();

            let arr = [
                first_start_zone,
                first_end_zone,
                second_start_zone,
                second_end_zone,
            ];

            let max = arr.iter().max().unwrap();

            if max > &last_zone {
                last_zone = *max;
            }

            return (
                Job::new(first_start_zone, first_end_zone),
                Job::new(second_start_zone, second_end_zone),
            );
        })
        .collect::<Vec<(Job, Job)>>();

    let complete_overlaps = pairs
        .iter()
        .filter(|(first, second)| {
            (first.start_zone <= second.start_zone && first.end_zone >= second.end_zone)
                || (second.start_zone <= first.start_zone && second.end_zone >= first.end_zone)
        })
        .count();

    println!("Part 1: {:?}", complete_overlaps);

    let partial_overlaps = pairs
        .iter()
        .filter(|(first, second)| {
            (first.start_zone <= second.start_zone && first.end_zone >= second.start_zone)
                || (second.start_zone <= first.start_zone && second.end_zone >= first.start_zone)
        })
        .count();

    println!("Part 1: {:?}", partial_overlaps);
}
