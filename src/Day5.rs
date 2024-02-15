use std::io::{BufReader, BufRead};
use std::fs::File;
use std::fmt;

#[derive(Clone)]
struct Interval {
    start: u64,
    stop: u64,
}

impl Interval {
    fn intersect(&self, temp_inter: Interval) -> Option<Interval> {
        let mut intersection_interval: Interval = Interval { start: 0,stop: 0};
        intersection_interval.start = self.start.max(temp_inter.start);
        intersection_interval.stop = self.stop.min(temp_inter.stop);
        if intersection_interval.start <= intersection_interval.stop {
            Some(intersection_interval)
        } else {
            None
        }
    }
    fn difference(&self, temp_inter: &Interval) -> Option<Vec<Interval>> {
        let mut difference: Vec<Interval> = Vec::new();
        if self.stop == temp_inter.stop {
            let my_interval = Interval { start: self.start, stop: temp_inter.start };
            difference.push(my_interval);
        } else if self.start == temp_inter.start {
            let my_interval = Interval { start: self.stop, stop: temp_inter.stop };
            difference.push(my_interval);
        } else {
           return None;
        }
        Some(difference)
    }
}

struct Map {
    source: u64,
    destination: u64,
    range: u64,
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Map {{source: {},destination: {}, range: {} }}",
            self.source, self.destination, self.range
        )
    }
}


fn main() {
    let file = File::open("xxx")
        .expect("File not found!");
    let reader = BufReader::new(file);
    let input: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    let mut real_maps_combined = Vec::new();

    let mut seeds = get_seeds(&input);
    let maps = get_maps(&input);

    let real_map1: Vec<Map> = create_maps(&maps.0);
    let real_map2: Vec<Map> = create_maps(&maps.1);
    let real_map3: Vec<Map> = create_maps(&maps.2);
    let real_map4: Vec<Map> = create_maps(&maps.3);
    let real_map5: Vec<Map> = create_maps(&maps.4);
    let real_map6: Vec<Map> = create_maps(&maps.5);
    let real_map7: Vec<Map> = create_maps(&maps.6);

    real_maps_combined.push(real_map1);
    real_maps_combined.push(real_map2);
    real_maps_combined.push(real_map3);
    real_maps_combined.push(real_map4);
    real_maps_combined.push(real_map5);
    real_maps_combined.push(real_map6);
    real_maps_combined.push(real_map7);

    let mut steps = Vec::new();
    let mut locations: Vec<u64> = Vec::new();

    seeds.reverse();

    for seed in &seeds {
        let mut cur_num = *seed;
        for m in &real_maps_combined {
            for am in m {
                if am.source <= cur_num && cur_num < am.source + am.range {
                    cur_num = am.destination + (cur_num - am.source);
                    steps.push(cur_num);
                    break;
                }
            }
        }
        if let Some(last_element) = steps.last().cloned() {
            locations.push(last_element);
            steps.clear();
        }
    }

    locations.sort();
    // for loc in &locations {
    //     println!("{:?}", loc);
    // }

    let new_seeds = compute_seed_intervals(seeds);
    let mut answers: Vec<Interval> = Vec::new();



}

fn map(maps: &[Vec<Map>], level: u64, mut my_interval: Interval, answers: &mut Vec<Interval>) {
    if level == 7 {
        answers.push(my_interval);
        return;
    }

    for mp in &maps[level as usize] {
        let map_interval = Interval {start: mp.source, stop: mp.source + mp.range - 1};
        if let Some(intersect) = my_interval.intersect(map_interval) {
            map(maps, level + 1, intersect, answers);
            if let Some(diff) = my_interval.difference(&intersect) {
                if diff.is_empty(){
                    return;
                }
                my_interval = diff[0].clone();
            }
        }
        else{
            return;
        }
    }
}


fn remap(rmb: &[Vec<Vec<Map>>], seed_ranges: &[Interval], answers: &mut Vec<Interval>) {
    for maps in rmb {
        for sr in seed_ranges {
            map(maps, 0, sr.clone(), answers); // Cloning `sr` to avoid borrowing issues
        }
    }
}

fn compute_seed_intervals(seeds: Vec<u64>) -> Vec<Interval> {
    let mut new_seeds: Vec<Interval> = Vec::new();
    for chunk in seeds.chunks(2) {
        if chunk.len() == 2 {
            let start = chunk[0];
            let range = chunk[1];
            let result = start + range - 1;
            let mut new_interval: Interval = Interval {start: 0, stop: 0};
            new_interval.start = start;
            new_interval.stop = result;
            new_seeds.push(new_interval);
        }
    }
    new_seeds
}

fn create_maps(num_map: &Vec<u64>) -> Vec<Map> {
    let mut temp_vec: Vec<Map> = Vec::new();
    let mut iter = num_map.iter();

    while let Some(&range) = iter.next() {
        if let (Some(&source), Some(&destination)) = (iter.next(), iter.next()) {
            let map = Map {
                source,
                destination,
                range,
            };
            temp_vec.push(map);
        } else {
            println!("Invalid number of elements in a chunk!");
            break;
        }
    }
    temp_vec
}

fn get_maps(input: &Vec<String>) -> (Vec<u64>, Vec<u64>, Vec<u64>, Vec<u64>, Vec<u64>, Vec<u64>, Vec<u64>) {
    let mut seed_to_soil: Vec<u64> = Vec::new();
    let mut soil_to_fertilizer: Vec<u64> = Vec::new();
    let mut fertilizer_to_water: Vec<u64> = Vec::new();
    let mut water_to_light: Vec<u64> = Vec::new();
    let mut light_to_temperature: Vec<u64> = Vec::new();
    let mut temperature_to_humidity: Vec<u64> = Vec::new();
    let mut humidity_to_location: Vec<u64> = Vec::new();
    let mut lines_iter = input.iter().peekable();
    let mut sum: u32 = 0;
    let mut counter: u32 = 0;

    while let Some(line) = lines_iter.next() {
        if line.contains("soil map:") {
            while let Some(next_line) = lines_iter.next() {
                if next_line.is_empty() {
                    break;
                }
                for char in next_line.chars().rev() {
                    if char.is_digit(10) {
                        let digit = char.to_digit(10).unwrap();
                        sum += digit * 10u32.pow(counter);
                        counter += 1;
                    } else if char == ' ' {
                        seed_to_soil.push(sum as u64);
                        sum = 0;
                        counter = 0;
                    }
                }
                seed_to_soil.push(sum as u64);
                sum = 0;
                counter = 0;
            }
        }
        if line.contains("fertilizer map:") {
            while let Some(next_line) = lines_iter.next() {
                if next_line.is_empty() {
                    break;
                }
                for char in next_line.chars().rev() {
                    if char.is_digit(10) {
                        let digit = char.to_digit(10).unwrap();
                        sum += digit * 10u32.pow(counter);
                        counter += 1;
                    } else if char == ' ' {
                        soil_to_fertilizer.push(sum as u64);
                        sum = 0;
                        counter = 0;
                    }
                }
                soil_to_fertilizer.push(sum as u64);
                sum = 0;
                counter = 0;
            }
        }
        if line.contains("water map:") {
            while let Some(next_line) = lines_iter.next() {
                if next_line.is_empty() {
                    break;
                }
                for char in next_line.chars().rev() {
                    if char.is_digit(10) {
                        let digit = char.to_digit(10).unwrap();
                        sum += digit * 10u32.pow(counter);
                        counter += 1;
                    } else if char == ' ' {
                        fertilizer_to_water.push(sum as u64);
                        sum = 0;
                        counter = 0;
                    }
                }
                fertilizer_to_water.push(sum as u64);
                sum = 0;
                counter = 0;
            }
        }
        if line.contains("light map:") {
            while let Some(next_line) = lines_iter.next() {
                if next_line.is_empty() {
                    break;
                }
                for char in next_line.chars().rev() {
                    if char.is_digit(10) {
                        let digit = char.to_digit(10).unwrap();
                        sum += digit * 10u32.pow(counter);
                        counter += 1;
                    } else if char == ' ' {
                        water_to_light.push(sum as u64);
                        sum = 0;
                        counter = 0;
                    }
                }
                water_to_light.push(sum as u64);
                sum = 0;
                counter = 0;
            }
        }
        if line.contains("temperature map:") {
            while let Some(next_line) = lines_iter.next() {
                if next_line.is_empty() {
                    break;
                }
                for char in next_line.chars().rev() {
                    if char.is_digit(10) {
                        let digit = char.to_digit(10).unwrap();
                        sum += digit * 10u32.pow(counter);
                        counter += 1;
                    } else if char == ' ' {
                        light_to_temperature.push(sum as u64);
                        sum = 0;
                        counter = 0;
                    }
                }
                light_to_temperature.push(sum as u64);
                sum = 0;
                counter = 0;
            }
        }
        if line.contains("humidity map:") {
            while let Some(next_line) = lines_iter.next() {
                if next_line.is_empty() {
                    break;
                }
                for char in next_line.chars().rev() {
                    if char.is_digit(10) {
                        let digit = char.to_digit(10).unwrap();
                        sum += digit * 10u32.pow(counter as u32);
                        counter += 1;
                    } else if char == ' ' {
                        temperature_to_humidity.push(sum as u64);
                        sum = 0;
                        counter = 0;
                    }
                }
                temperature_to_humidity.push(sum as u64);
                sum = 0;
                counter = 0;
            }
        }
        if line.contains("location map:") {
            while let Some(next_line) = lines_iter.next() {
                if next_line.is_empty() {
                    break;
                }
                for char in next_line.chars().rev() {
                    if char.is_digit(10) {
                        let digit = char.to_digit(10).unwrap();
                        sum += digit * 10u32.pow(counter);
                        counter += 1;
                    } else if char == ' ' {
                        humidity_to_location.push(sum as u64);
                        sum = 0;
                        counter = 0;
                    }
                }
                humidity_to_location.push(sum as u64);
                sum = 0;
                counter = 0;
            }
        }
    }
    (seed_to_soil, soil_to_fertilizer, fertilizer_to_water, water_to_light, light_to_temperature, temperature_to_humidity, humidity_to_location)
}

fn get_seeds(input: &Vec<String>) -> Vec<u64> {
    let mut seeds: Vec<u64> = Vec::new();
    let mut sum: u32 = 0;
    let mut counter: u32 = 0;

    for char in input[0].chars().rev() {
        if char.is_digit(10) {
            let digit = char.to_digit(10).unwrap();
            sum += digit * 10u32.pow(counter);
            counter += 1;
        } else if char == ' ' {
            seeds.push(sum as u64);
            sum = 0;
            counter = 0;
        }
    }

    seeds
}
