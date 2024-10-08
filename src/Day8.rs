use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashMap;

fn main() {
    let file = File::open("input_day_8.txt")
        .expect("File not found!");
    let reader = BufReader::new(file);
    let input: Vec<String> = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| !line.is_empty())
        .collect();

    let mut leafs: Vec<(String, (String, String))> = Vec::new();
    let mut cleaned_args: Vec<String> = Vec::new();
    for line in input.iter() {
        let args: Vec<String> = line.split_whitespace().map(String::from).collect();
        for arg in args.into_iter().map(|arg| arg.replace("(", "").replace(")", "").replace(",", "")) {
            if arg != "=".to_string() {
                cleaned_args.push(arg);
            }
        }
    }
    let path = cleaned_args[0].clone();
    for window in cleaned_args[1..].windows(3).step_by(3) {
        if let [rootleaf, leftleaf, rightleaf] = window {
            leafs.push((rootleaf.to_string(), (leftleaf.to_string(), rightleaf.to_string())));
        }
    }

    let mut myMap: HashMap<String, (String, String)> = HashMap::new();
    let mut part2_current_nodes = Vec::new();

    for leaf in leafs.into_iter() {
        let mut inter_leaf = leaf.clone();
        myMap.insert(leaf.0, leaf.1);
        if inter_leaf.0.chars().last().unwrap() == 'A' {
            part2_current_nodes.push(inter_leaf);
        }
    }
    part_1(&myMap,&path);
    part_2(&myMap,&path,part2_current_nodes);
}

fn part_2(myMap:&HashMap<String,(String,String)>,path:&String,part2_current_nodes:Vec<(String,(String,String))>) {

    let mut steps = Vec::new();
    for elem in part2_current_nodes.into_iter() {
        steps.push(calculate_steps(&myMap, &path, elem.0));
    }
    let new_steps: Vec<u64> = steps.iter().map(|&x| x as u64).collect();
    println!("{}", lcm_of_list(&new_steps));
}
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}
fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}
fn lcm_of_list(numbers: &[u64]) -> u64 {
    numbers.iter().cloned().reduce(lcm).unwrap()
}
fn calculate_steps(myMap: &HashMap<String, (String, String)>, path: &String, mut current_node: String) -> i32 {
    let mut count: i32 = 0;
    while current_node.chars().last() != Some('Z') {
        let mut step = path.chars().nth(count as usize % path.len()).unwrap();
        if step == 'L' {
            current_node = myMap[&current_node].0.clone();
        } else {
            current_node = myMap[&current_node].1.clone();
        }
        count += 1
    }
    count
}
fn part_1(myMap: &HashMap<String, (String, String)>, path: &String) {
    let mut current_node = "AAA";
    let mut count = 0;
    while current_node != "ZZZ" {
        let mut step = path.chars().nth(count % path.len()).unwrap();
        if step == 'L' {
            current_node = &myMap[current_node].0;
        } else {
            current_node = &myMap[current_node].1;
        }
        count += 1
    }
    println!("{}", count);
}