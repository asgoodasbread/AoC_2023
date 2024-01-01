use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn main() {
    part1();
    part2();
}
fn part2() {
    let file = File::open("inputs/day_2_input.txt").expect("File not found !");
    let reader = BufReader::new(file);
    let mut highest_amount_of: Vec<u32> = Vec::new();
    let mut color: Vec<char> = Vec::new();
    let mut result_part_2: u32 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let parts: Vec<&str> = line.splitn(2, ':').collect();
        let mut final_string: String = String::new();
        final_string = parts[1].replace(";", "");
        final_string = final_string.chars().filter(|&c| !c.is_whitespace()).collect();

        let mut iter = final_string.chars();
        while let Some(char) = iter.next() {
            if char.is_digit(10) {
                if let Some(next_char) = iter.next() {
                    if next_char.is_digit(10) {
                        let mut two_digits = format!("{}{}", char, next_char);
                        if let Ok(number) = two_digits.parse::<u32>() {
                            highest_amount_of.push(number);
                        }
                        if let Some(next_char) = iter.next() {
                            if next_char.is_alphabetic() {
                                color.push(next_char);
                            }
                        }
                    }
                    if next_char.is_alphabetic() {
                        highest_amount_of.push(char as u32 - 48);
                        color.push(next_char);
                    }
                }
            }
        }
        let mut max_blue = 0;
        let mut max_red = 0;
        let mut max_green = 0;

        for i in 0..color.len() {
            if color[i] == 'b' && highest_amount_of[i] > max_blue {
                max_blue = highest_amount_of[i];
            }
            if color[i] == 'r' && highest_amount_of[i] > max_red {
                max_red = highest_amount_of[i];
            }
            if color[i] == 'g' && highest_amount_of[i] > max_green {
                max_green = highest_amount_of[i];
            }
        }
        result_part_2 += max_blue*max_red*max_green;
        highest_amount_of.clear();
        color.clear();
    }
    println!("Result part 2: {}",result_part_2);
}

fn part1() {
    let file = File::open("inputs/day_2_input.txt").expect("File not found !");
    let reader = BufReader::new(file);
    let mut game_id: u32 = 0;
    let mut color_counter: Vec<u8> = Vec::new();
    let mut colors: Vec<char> = Vec::new();
    let mut result_part_1: u32 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        game_id += 1;
        let parts: Vec<&str> = line.splitn(2, ':').collect();
        let mut final_string: String = String::new();
        final_string = parts[1].replace(";", "");
        final_string = final_string.chars().filter(|&c| !c.is_whitespace()).collect();

        let mut iter = final_string.chars();

        while let Some(char) = iter.next() {
            if char.is_digit(10) {
                if let Some(next_char) = iter.next() {
                    if next_char.is_digit(10) {
                        let mut two_digits = format!("{}{}", char, next_char);
                        if let Ok(number) = two_digits.parse::<u8>() {
                            color_counter.push(number);
                        }
                        if let Some(next_char) = iter.next() {
                            if next_char.is_alphabetic() {
                                colors.push(next_char);
                            }
                        }
                    }
                }
            }
        }

        if colors.is_empty() {
            result_part_1 += game_id;
        }
        for i in 0..colors.len() {
            if color_counter[i] > 12 && colors[i] == 'r' {
                color_counter.clear();
                colors.clear();
                break;
            } else if color_counter[i] > 13 && colors[i] == 'g' {
                color_counter.clear();
                colors.clear();
                break;
            } else if color_counter[i] > 14 && colors[i] == 'b' {
                color_counter.clear();
                colors.clear();
                break;
            } else if i == colors.len() - 1 {
                result_part_1 += game_id;
                color_counter.clear();
                colors.clear();
                break;
            }
        }
    }
    println!("Result part 1: {}", result_part_1);
}

