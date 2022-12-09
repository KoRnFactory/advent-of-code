pub fn solve() {
    let input = include_str!("day8.input.txt");

    let grid = input
        .lines()
        .map(|row| {
            row.split("")
                .flat_map(|tree| tree.parse::<usize>())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let mut visibility_map = grid
        .iter()
        .map(|row| row.iter().map(|_| false).collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>();

    for (i, row) in grid.iter().enumerate() {
        for (j, &tree) in row.iter().enumerate() {
            visibility_map[i][j] = if i == 0 || i == grid.len() - 1 || j == 0 || j == row.len() - 1
            {
                true
            } else {
                (0..j).all(|index| grid[i][index] < tree)
                    || ((j + 1)..row.len()).all(|index| grid[i][index] < tree)
                    || (0..i).all(|index| grid[index][j] < tree)
                    || ((i + 1)..grid.len()).all(|index| grid[index][j] < tree)
            };
        }
    }

    println!(
        "Part1: {:?}",
        visibility_map
            .iter()
            .map(|row| row.iter().filter(|&&item| item == true).count())
            .sum::<usize>()
    );

    let mut scenic_score_map = grid
        .iter()
        .map(|row| row.iter().map(|_| 0u32).collect::<Vec<u32>>())
        .collect::<Vec<Vec<u32>>>();

    for (i, row) in grid.iter().enumerate() {
        for (j, &tree) in row.iter().enumerate() {
            scenic_score_map[i][j] =
                if i == 0 || i == grid.len() - 1 || j == 0 || j == row.len() - 1 {
                    0
                } else {
                    let mut left_score = 0u32;
                    let mut right_score = 0u32;
                    let mut top_score = 0u32;
                    let mut bottom_score = 0u32;

                    for index in (0..j).rev() {
                        left_score += 1;
                        if grid[i][index] >= tree {
                            break;
                        }
                    }

                    for index in (j + 1)..row.len() {
                        right_score += 1;
                        if grid[i][index] >= tree {
                            break;
                        }
                    }

                    for index in (0..i).rev() {
                        top_score += 1;
                        if grid[index][j] >= tree {
                            break;
                        }
                    }

                    for index in (i + 1)..grid.len() {
                        bottom_score += 1;
                        if grid[index][j] >= tree {
                            break;
                        }
                    }

                    left_score * right_score * top_score * bottom_score
                };
        }
    }

    println!(
        "Part2: {:?}",
        scenic_score_map
            .iter()
            .map(|row| row.iter().max().unwrap())
            .max()
            .unwrap()
    );
}
