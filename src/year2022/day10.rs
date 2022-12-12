pub fn solve() {
    let input = include_str!("day10.input.txt");

    let mut cycles = 0;
    let mut register_x = 1;
    let mut signal_strengths: Vec<i32> = vec![];
    let mut screen: Vec<&str> = vec![];

    for line in input.lines() {
        let mut tokens = line.split(" ");
        let cmd = tokens.next().expect("Input malformed");
        let cycles_taken: u32 = match cmd {
            "noop" => 1,
            "addx" => 2,
            _ => unreachable!(),
        };

        for current_cycle in (0..cycles_taken).rev() {
            let valid_pixel = ((register_x - 1)..=(register_x + 1))
                .into_iter()
                .find(|&position| position == cycles % 40);

            screen.push(match valid_pixel {
                None => ".",
                Some(_) => "#",
            });

            cycles += 1;

            if (cycles - 20) % 40 == 0 {
                signal_strengths.push(calculate_signal_strength(cycles, register_x))
            }

            if current_cycle == 0 {
                match cmd {
                    "addx" => {
                        let amount = tokens
                            .next()
                            .unwrap()
                            .parse::<i32>()
                            .expect("Input malformed");
                        register_x += amount;
                    }
                    _ => {}
                }
            }
        }
    }

    println!("Part 1: {:?}", signal_strengths.iter().sum::<i32>());

    println!("Part 2:");

    for (index, &value) in screen.iter().enumerate() {
        if (index + 1) % 40 == 0 {
            println!("{}", value);
        } else {
            print!("{}", value);
        }
    }
}

fn calculate_signal_strength(cycle: i32, register: i32) -> i32 {
    cycle * register
}
