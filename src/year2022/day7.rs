use itertools::Itertools;

const TOTAL_SPACE: usize = 70000000;

#[derive(Debug)]
struct Directory {
    subdirectories: Vec<Directory>,
    files: Vec<(String, usize)>,
}

impl Directory {
    fn new() -> Self {
        Self {
            subdirectories: vec![],
            files: vec![],
        }
    }

    fn add_subdirectory(&mut self, subdirectory: Directory) {
        self.subdirectories.push(subdirectory);
    }

    fn add_file(&mut self, name: String, size: usize) {
        self.files.push((name, size));
    }

    fn get_size(&self) -> usize {
        let file_sizes = self.files.iter().map(|(_, size)| size).sum::<usize>();
        let dir_sizes = self.subdirectories.iter().map(|dir| dir.get_size()).sum::<usize>();

        file_sizes + dir_sizes
    }

    fn sizes(&self) -> Vec<usize> {
        self.subdirectories
            .iter()
            .flat_map(|dir| dir.sizes())
            .chain([self.get_size()])
            .collect_vec()
    }
}

pub fn solve() {
    let input = include_str!("day7.input.txt");

    let mut cwd = vec![Directory::new()];

    for line in input.lines() {
        match line {
            "$ ls" => {}
            "$ cd /" => {}
            "$ cd .." => {
                let directory = cwd.pop().unwrap();
                cwd.last_mut().unwrap().add_subdirectory(directory);
            }
            line if line.starts_with("$ cd") => {
                cwd.push(Directory::new())
            }
            line if line.starts_with("dir") => {}
            line => {
                let (file_name, size) = line.split_once(" ").map(|(size, name)|
                    (name.to_string(), size.parse::<usize>().expect("File size is wrong"))
                ).expect("Parse error");
                cwd.last_mut().unwrap().add_file(file_name, size);
            }
        }
    }

    while cwd.len() > 1 {
        let directory = cwd.pop().unwrap();
        cwd.last_mut().unwrap().add_subdirectory(directory);
    }

    let dirs = cwd.iter().next().unwrap().sizes();

    println!("Part 1: {:?}", dirs.iter().filter(|&&size| size < 100000).sum::<usize>());

    let total_used_space = *dirs.iter().last().unwrap();
    let needed_space = 30000000 - (TOTAL_SPACE - total_used_space);

    println!("Part 2: {:?}", dirs.iter().filter(|&&size| size >= needed_space).min().unwrap());
}
