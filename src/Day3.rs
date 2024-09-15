use std::io::{BufRead, BufReader};
use std::fs::File;
use std::fmt;

struct Number {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
    value: i32,
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Use the formatter to write the formatted string
        write!(
            f,
            "Number: ({}, {}), ({}, {}) - Value: {}",
            self.x1, self.y1, self.x2, self.y2, self.value
        )
    }
}

fn main() {
    part_1();
    part_2();
}

fn check_if_belongs(myNumber: &Number, star_cords: (i32, i32)) -> bool {
    let star_x = star_cords.0;
    let star_y = star_cords.1;
    if (myNumber.x1 - star_x).abs() == 1 {
        if (myNumber.y1 - 1 <= star_y && star_y <= myNumber.y2 + 1) {
            return true;
        }
    } else if (myNumber.x1 == star_x) {
        if (myNumber.y1 - star_y).abs() == 1 || (myNumber.y2 - star_y).abs() == 1 {
            return true;
        }
    }
    return false;
}

fn find_numbers(input: &Vec<Vec<char>>) -> Vec<Number> {
    let mut number_cords: Vec<Number> = Vec::new();
    let mut current_sum: i32 = 0;
    let mut prev_num_counter = 0;
    let mut last_x = 0;
    let mut last_y = 0;
    for row in 0..input.len() {
        for column in (0..input[0].len()).rev() {
            let mut element = input[row][column];
            if element.is_digit(10) {
                let mut current_number: i32 = element.to_digit(10).unwrap() as i32;
                current_sum += current_number * 10i32.pow(prev_num_counter);
                prev_num_counter += 1;
                if prev_num_counter == 1 {
                    last_x = row;
                    last_y = column;
                }
            } else {
                if prev_num_counter > 0 {
                    let my_number = Number { x1: (row - ((column + 1) / input.len())) as i32, y1: ((column + 1) % input.len()) as i32, x2: last_x as i32, y2: last_y as i32, value: current_sum };
                    number_cords.push(my_number);
                    current_sum = 0;
                    prev_num_counter = 0;
                }
            }
        }
    }
    return number_cords;
}

fn part_2() {
    let file = File::open("inputs/day_3_input.txt").expect("File not found !");
    let reader = BufReader::new(file);
    let mut input: Vec<Vec<char>> = Vec::new();
    let mut result_part_2: i32 = 0;
    for line in reader.lines() {
        let mut row: Vec<char> = Vec::new();
        for element in line {
            for ch in element.chars() {
                row.push(ch);
            }
        }
        input.push(row);
    }
    //part_1();

    let check = find_numbers(&input);

    let stars = star_finder(&input);

    let mut belongs: Vec<i32> = Vec::new();

    for &star in &stars {
        for number in &check {
            if check_if_belongs(&number, star) {
                belongs.push(number.value);
            }
        }
        if belongs.len() == 2 {
            let mut sum = belongs[0] * belongs[1];
            result_part_2 += sum;
            sum = 0;
        }
        belongs.clear();
    }
    println!("Result part 2: {}", result_part_2);
}

fn star_finder(input: &Vec<Vec<char>>) -> Vec<(i32, i32)> {
    //find coordinates of a star
    let mut star_cords: Vec<(i32, i32)> = Vec::new();
    let mut x = 0;
    let mut y = 0;
    for row in input {
        for elem in row {
            if *elem == '*' {
                star_cords.push((x, y))
            }
            if y == 140 {
                y = 0;
            }
            y += 1;
        }
        x += 1;
    }
    return star_cords;
}

fn check_adjacent(matrix: &Vec<Vec<char>>, cur_row: usize, cur_col: usize) -> bool {
    let symbols: Vec<char> = vec!['@', '/', '*', '^', '!', '#', '$', ',', '+', '-', '?', '=', '&', '%'];
    let directions = vec![(0, 1), (0, -1), (1, 0), (-1, 0), (1, 1), (1, -1), (-1, 1), (-1, -1)];
    //Matrix indices are of type usize
    let current_char = matrix[cur_row][cur_col];

    if current_char.is_digit(10) {
        for (row_change, col_change) in directions.iter() {
            let new_row = cur_row as isize + row_change;
            let new_col = cur_col as isize + col_change;
            if new_row >= 0 && new_row < matrix.len() as isize && new_col >= 0 && new_col < matrix[0].len() as isize {
                let new_row = new_row as usize;
                let new_col = new_col as usize;
                let adjacent_char = matrix[new_row][new_col];
                if symbols.contains(&adjacent_char) {
                    return true;
                }
            }
        }
    }
    return false;
}

fn part_1() {
    let file = File::open("inputs/day_3_input.txt").expect("File not found !");
    let reader = BufReader::new(file);

    let mut input: Vec<Vec<char>> = Vec::new();
    let mut result_part_1: u32 = 0;

    for line in reader.lines() {
        let mut row: Vec<char> = Vec::new();
        for element in line {
            for ch in element.chars() {
                row.push(ch);
            }
        }
        input.push(row);
    }
    let mut row_iter = input.iter().enumerate();

    while let Some((row_index, row_val)) = row_iter.next() {
        let mut col_iter = row_val.iter().enumerate();
        while let Some((col_index, col_val)) = col_iter.next() {
            if !col_val.is_digit(10) {
                continue;
            }
            let Some((next_col_index, next_col_val)) = col_iter.next() else {
                continue;
            };
            if next_col_val.is_digit(10) {
                let Some((next_next_col_index, next_next_col_val)) = col_iter.next() else {
                    continue;
                };
                if next_next_col_val.is_digit(10) {
                    if check_adjacent(&input, row_index, col_index) ||
                        check_adjacent(&input, row_index, next_col_index) ||
                        check_adjacent(&input, row_index, next_next_col_index) {
                        let col_val = col_val.to_digit(10).unwrap();
                        let next_col_val = next_col_val.to_digit(10).unwrap();
                        let next_next_col_val = next_next_col_val.to_digit(10).unwrap();
                        result_part_1 += 100 * col_val + 10 * next_col_val + next_next_col_val;
                    }
                } else {
                    if check_adjacent(&input, row_index, col_index) || check_adjacent(&input, row_index, next_col_index) {
                        let col_val = col_val.to_digit(10).unwrap();
                        let next_col_val = next_col_val.to_digit(10).unwrap();
                        result_part_1 += 10 * col_val + next_col_val;
                    }
                }
            } else {
                if check_adjacent(&input, row_index, col_index) {
                    result_part_1 += col_val.to_digit(10).unwrap();
                }
            }
        }
    }

    for row in &input {
        for column in row {
            if column.is_digit(10) {
                let _ = column.to_digit(10);
            }
        }
    }


    // for row in &input {
    //     for elem in row {
    //         print!("{} ", elem);
    //     }
    //     println!();
    // }

    //println!("Number of rows: {}", &input.len());
    //println!("Number of chars is a row: {}", &input[0].len(), );

    let matrix: Vec<Vec<char>> = vec![
        vec!['a', 'g', '/'],
        vec!['d', '6', 'f'],
        vec!['g', 'h', 'i'],
    ];

    // Check adjacent fields for the cell at (1, 1)
    let result = check_adjacent(&matrix, 2, 0);
    //println!("{}", result);
    println!("Result part 1: {}", result_part_1);
}


fn cos(input: &Vec<Vec<char>>) {
    let mut digits: Vec<u32> = Vec::new();
    for row in 0..input.len() {
        for column in 0..input[0].len() {
            let mut zmienna = input[row][column];
            if zmienna.is_digit(10) {
                digits.push(zmienna.to_digit(10).unwrap());
            } else if digits.len() == 0 {
                continue;
            } else {
                let mut multiplier = 1;
                let mut sum = 0;
                for digit in digits.iter().rev() {
                    sum += digit * multiplier;
                    multiplier *= 10;
                }
                digits.clear();
                println!("{}", sum);
            }
        }
    }
}



