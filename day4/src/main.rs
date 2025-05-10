use std::io;

fn main() {
    let mut grid = Vec::new();

    // Reading grid input
    loop {
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim().to_string();

        if input.is_empty() {
            break;
        }
        let row: Vec<char> = input.chars().collect();
        grid.push(row);
    }
    part2(grid)
}

fn part1(grid: Vec<Vec<char>>) {
    let word = "XMAS";
    let directions: Vec<(i32, i32)> = vec![
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ];

    let word_len = word.len() as i32;
    let row_len = grid.len() as i32;
    let column_count = grid.iter().map(|row| row.len()).max().unwrap_or(0) as i32;

    let mut word_counter = 0;

    for row_index in 0..row_len {
        for column_index in 0..column_count {
            for &(dir_x, dir_y) in &directions {
                let mut match_found = true;

                for (index, character) in word.chars().enumerate() {
                    let new_row = row_index + index as i32 * dir_x;
                    let new_col = column_index + index as i32 * dir_y;

                    if new_row < 0 || new_row >= row_len || new_col < 0 || new_col >= column_count {
                        match_found = false;
                        break;
                    }

                    if grid[new_row as usize][new_col as usize] != character {
                        match_found = false;
                        break;
                    }
                }

                if match_found {
                    println!(
                        "Word found at ({}, {}) in direction ({}, {})",
                        row_index, column_index, dir_x, dir_y
                    );
                    word_counter += 1;
                }
            }
        }
    }

    println!("{:?}", grid);
    println!("Rows: {}, Columns: {}", row_len, column_count);
    println!("Word occurrences: {}", word_counter);
}

fn part2(grid: Vec<Vec<char>>) {
    let word = "MAS";
    let directions: Vec<(i32, i32)> = vec![(1, 1), (-1, -1), (1, -1), (-1, 1)];

    let word_len = word.len() as i32;
    let row_len = grid.len() as i32;
    let column_count = grid.iter().map(|row| row.len()).max().unwrap_or(0) as i32;

    let mut word_counter = 0;

    for row_index in 0..row_len {
        for column_index in 0..column_count {
            let mut counter = 0;
            for &(dir_x, dir_y) in &directions {
                let mut match_found = true;

                for (index, character) in word.chars().enumerate() {
                    let new_row = row_index + (index as i32 - 1) * dir_x;
                    let new_col = column_index + (index as i32 - 1) * dir_y;

                    if new_row < 0 || new_row >= row_len || new_col < 0 || new_col >= column_count {
                        match_found = false;
                        break;
                    }

                    if grid[new_row as usize][new_col as usize] != character {
                        match_found = false;
                        break;
                    }
                }

                if match_found {
                    println!(
                        "Word found at ({}, {}) in direction ({}, {})",
                        row_index, column_index, dir_x, dir_y
                    );
                    counter += 1;
                }
            }
            if counter >= 2 {
                word_counter += 1
            }
        }
    }

    println!("Word occurrences: {}", word_counter);
}
