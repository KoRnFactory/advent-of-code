pub fn solve() {
    let input = include_str!("day2.input.txt");

    let rounds = input.lines();

    let inputs = rounds.map(|round| {
        let moves = round.split(" ").collect::<Vec<&str>>();
        let opponent = moves[0];
        let me = moves[1];

        return (opponent, me);
    });


    println!("Part 1: {:?}", inputs.clone().map(|(opponent, me)| calculate_round_part1(opponent, me)).sum::<u32>());
    println!("Part 2: {:?}", inputs.clone().map(|(opponent, result)| calculate_round_part2(opponent, result)).sum::<u32>());
}

fn calculate_round_part1(opponent_move: &str, my_move: &str) -> u32 {
    let opponent_move = convert_move(opponent_move).unwrap();
    let my_move = convert_move(my_move).unwrap();

    let my_score: u32 = calculate_move_score(my_move);

    let result = calculate_result(opponent_move, my_move);
    let result_score = calculate_result_score(result);

    return my_score + result_score;
}


fn calculate_round_part2(opponent_move: &str, expected_result: &str) -> u32 {
    let opponent_move = convert_move(opponent_move).unwrap();
    let expected_result = convert_result(expected_result).unwrap();

    let my_move = choose_move(opponent_move, expected_result);
    let my_score: u32 = calculate_move_score(my_move);

    let result = calculate_result(opponent_move, my_move);
    let result_score = calculate_result_score(result);

    return my_score + result_score;
}

fn calculate_result_score(result: GameResult) -> u32 {
    match result {
        GameResult::Win => 6,
        GameResult::Draw => 3,
        _ => 0
    }
}

fn calculate_move_score(my_move: GameMove) -> u32 {
    match my_move {
        GameMove::Rock => 1,
        GameMove::Paper => 2,
        GameMove::Scissors => 3,
    }
}

fn calculate_result(opponent_move: GameMove, my_move: GameMove) -> GameResult {
    match opponent_move {
        GameMove::Rock => match my_move {
            GameMove::Rock => { GameResult::Draw }
            GameMove::Paper => { GameResult::Win }
            GameMove::Scissors => { GameResult::Loss }
        },
        GameMove::Paper => match my_move {
            GameMove::Rock => { GameResult::Loss }
            GameMove::Paper => { GameResult::Draw }
            GameMove::Scissors => { GameResult::Win }
        },
        GameMove::Scissors => match my_move {
            GameMove::Rock => { GameResult::Win }
            GameMove::Paper => { GameResult::Loss }
            GameMove::Scissors => { GameResult::Draw }
        },
    }
}

fn convert_move(input: &str) -> Result<GameMove, ()> {
    match input {
        "A" | "X" => Ok(GameMove::Rock),
        "B" | "Y" => Ok(GameMove::Paper),
        "C" | "Z" => Ok(GameMove::Scissors),
        _ => { Err(()) }
    }
}

fn convert_result(input: &str) -> Result<GameResult, ()> {
    match input {
        "X" => Ok(GameResult::Loss),
        "Y" => Ok(GameResult::Draw),
        "Z" => Ok(GameResult::Win),
        _ => { Err(()) }
    }
}

fn choose_move(opponent_move: GameMove, expected_result: GameResult) -> GameMove {
    match expected_result {
        GameResult::Win => {
            match opponent_move {
                GameMove::Rock => { GameMove::Paper }
                GameMove::Paper => { GameMove::Scissors }
                GameMove::Scissors => { GameMove::Rock }
            }
        }
        GameResult::Draw => { opponent_move }
        GameResult::Loss => {
            match opponent_move {
                GameMove::Rock => { GameMove::Scissors }
                GameMove::Paper => { GameMove::Rock }
                GameMove::Scissors => { GameMove::Paper }
            }
        }
    }
}

#[derive(Clone, Copy)]
enum GameResult {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

#[derive(Clone, Copy)]
enum GameMove {
    Rock,
    Paper,
    Scissors,
}
