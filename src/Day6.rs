use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part_1();
    part_2();
}

fn part_2() {
    let file = File::open("day6_test_input.txt").expect("File not found !");
    let reader = BufReader::new(file);

    let mut time: i128 = 0;
    let mut distance: i128 = 0;

    for line in reader.lines() {
        if let Ok(mut line_content) = line {
            line_content.retain(|ch| !ch.is_whitespace());
            if line_content.contains("Time:") {
                let mut power = 0;
                let mut sum: u64 = 0;
                for ch in line_content.chars().rev() {
                    if ch.is_digit(10) {
                        sum += ch.to_digit(10).unwrap() as u64 * 10u64.pow(power);
                        power += 1;
                    } else {
                        if sum != 0 {
                            time = sum as i128;
                        }
                        sum = 0;
                        power = 0;
                    }
                }
            } else if line_content.contains("Distance:") {
                let mut power = 0;
                let mut sum = 0;
                for ch in line_content.chars().rev() {
                    if ch.is_digit(10) {
                        sum += ch.to_digit(10).unwrap() as u64 * 10u64.pow(power);
                        power += 1;
                    } else {
                        if sum != 0 {
                            distance = sum as i128;
                        }
                        sum = 0;
                        power = 0;
                    }
                }
            }
        } else {
            eprintln!("Error reading line from file");
        }
    }
    let mut results: Vec<i128> = Vec::new();
    let mut counter = 0;
    for tp in 1..time {
        let ttm = time - tp;
        let speed = tp;
        if speed * ttm > distance {
            counter += 1;
        }
    }
    results.push(counter);
    counter = 0;
    for elem in results {
        println!("{}", elem);
    }
}

fn part_1() {
    let file = File::open("day6_test_input.txt").expect("File not found !");
    let reader = BufReader::new(file);

    let mut times: Vec<i32> = Vec::new();
    let mut distances: Vec<i32> = Vec::new();

    //Read the values of all races into vectors
    for line in reader.lines() {
        if let Ok(line_content) = line {
            if line_content.contains("Time:") {
                let mut power = 0;
                let mut sum = 0;
                for ch in line_content.chars().rev() {
                    if ch.is_digit(10) {
                        sum += ch.to_digit(10).unwrap() * 10u32.pow(power);
                        power += 1;
                    } else {
                        if sum != 0 {
                            times.push(sum as i32);
                        }
                        sum = 0;
                        power = 0;
                    }
                }
            }
            if line_content.contains("Distance:") {
                let mut power = 0;
                let mut sum = 0;
                for ch in line_content.chars().rev() {
                    if ch.is_digit(10) {
                        sum += ch.to_digit(10).unwrap() * 10u32.pow(power);
                        power += 1;
                    } else {
                        if sum != 0 {
                            distances.push(sum as i32);
                        }
                        sum = 0;
                        power = 0;
                    }
                }
            }
        } else {
            eprintln!("Error reading line from file");
        }
    }
    times = times.iter().rev().cloned().collect();
    distances = distances.iter().rev().cloned().collect();

    let mut results = Vec::new();
    let mut counter = 0;
    //times.size() = distances.size()
    for i in 0..times.len() {
        for tp in 1..times[i] {
            let ttm = times[i] - tp;
            let speed = tp;
            if speed * ttm > distances[i] {
                counter += 1;
            }
        }
        results.push(counter);
        counter = 0;
    }

    let mut result_part1 = 1;

    for elem in results {
        result_part1 = result_part1 * elem;
    }

    println!("{}", result_part1);
}