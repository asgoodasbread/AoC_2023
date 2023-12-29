use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() {
    //part1();
    part2();
}

fn part2() {
    let file = File::open("inputs/day_1_input.txt").expect("File not found");
    let reader = BufReader::new(file);
    let mut result_part_2: u32 = 0;
    let mut combine_numbers: Vec<u32> = Vec::new();
    let (mut first_digit, mut last_digit) = (String::new(), String::new());

    let mut word_to_number = HashMap::new();
    word_to_number.insert("one", 1);
    word_to_number.insert("two", 2);
    word_to_number.insert("three", 3);
    word_to_number.insert("four", 4);
    word_to_number.insert("five", 5);
    word_to_number.insert("six", 6);
    word_to_number.insert("seven", 7);
    word_to_number.insert("eight", 8);
    word_to_number.insert("nine", 9);

    for line in reader.lines() {
        let line = line.unwrap();

        'outer:for char in line.chars() {
            if (char.is_alphabetic()) {
                first_digit.push(char);
                for (word, &value) in &word_to_number {
                    if first_digit.find(word).is_some() {
                        combine_numbers.push(value);
                        first_digit.clear();
                        break 'outer;
                    }
                }
            } else if (char.is_digit(10) && first_digit.is_empty()) {
                combine_numbers.push(char as u32 - 48);
                break;
            }
            else{
                first_digit.push(char);
                for char in first_digit.chars() {
                    if char.is_digit(10) {
                        combine_numbers.push(char as u32 - 48);
                        first_digit.clear();
                        break 'outer;
                    }
                }
            }
        }

        first_digit.clear();
        let mut reversed_string: String = String::new();

        'outer: for char in line.chars().rev() {
            if char.is_alphabetic() {
                last_digit.push(char);
                for (word, &value) in &word_to_number {
                    reversed_string = last_digit.chars().rev().collect();
                    if reversed_string.find(word).is_some() {
                        combine_numbers.push(value);
                        break 'outer;
                    }
                }
            } else if (char.is_digit(10) && last_digit.is_empty()) {
                combine_numbers.push(char as u32 - 48);
                break;
            }
            else{
                first_digit.push(char);
                for char in first_digit.chars() {
                    if char.is_digit(10) {
                        combine_numbers.push(char as u32 - 48);
                        first_digit.clear();
                        break 'outer;
                    }
                }
            }
        }
        last_digit.clear();
        reversed_string.clear();
    }

    let mut two_digit_numbers = Vec::new();

    for chunk in combine_numbers.chunks(2) {
        // Join the digits into a single number
        let digits = match chunk.len() {
            1 => chunk[0],            // If only one digit, use it as is
            2 => chunk[0] * 10 + chunk[1], // Combine two digits into a two-digit number
            _ => continue,            // Skip chunks with more than 2 digits
        };

        two_digit_numbers.push(digits);
    }

    //println!("{:?}", two_digit_numbers);

    result_part_2 = two_digit_numbers.iter().sum();
    println!("Result part 2: {}", result_part_2);

}


fn part1() {
    let file = File::open("inputs/day_1_input.txt").expect("File not found");
    let reader = BufReader::new(file);
    let mut result_part_1: u32 = 0;
    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        lines.push(line);
    }

    for line in &lines {
        if let Some(number) = combine_first_and_last_number(line) {
            result_part_1 += number;
        }
    }
    println!("{}", result_part_1);
}

fn combine_first_and_last_number(s: &str) -> Option<u32> {
    let (first_number, last_number) = find_first_and_last_number(s);

    match (first_number, last_number) {
        (Some(first), Some(last)) => Some(first * 10 + last),
        _ => None,
    }
}
fn find_first_and_last_number(s: &str) -> (Option<u32>, Option<u32>) {
    let mut first_number: Option<u32> = None;
    let mut last_number: Option<u32> = None;

    for c in s.chars() {
        if c.is_digit(10) {
            let digit = c.to_digit(10).unwrap();
            if first_number.is_none() {
                first_number = Some(digit);
            }
            last_number = Some(digit);
        }
    }
    (first_number, last_number)
}
