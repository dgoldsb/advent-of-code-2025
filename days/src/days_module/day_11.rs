use std::collections::{HashMap, HashSet};
use std::path::Path;
use crate::days_module::day::Day;

pub struct Day11 {}

fn get_neighbor_map(input: &String) -> HashMap<String, HashSet<String>> {
    let mut map: HashMap<String, HashSet<String>> = HashMap::new();
    for line in input.lines() {
        let split = line.split_once(": ").unwrap();
        let key = split.0.to_string();
        for value in split.1.split(" ") {
            map.entry(key.trim_end_matches(":").to_string()).or_insert_with(HashSet::new).insert(value.to_string());
        }
    }
    map
}

impl Day for Day11 {
    fn get_id(&self) -> String {
        "day_11".to_string()
    }

    fn get_index(&self) -> u8 {
        11
    }

    fn part_a(&self, input: &String) -> String {
        let map = get_neighbor_map(input);
        let mut stack = vec!["you"];
        let mut count = 0;

        while !stack.is_empty() {
            let current = stack.pop().unwrap();
            for next in map.get(current).unwrap() {
                if next == "out" {
                    count += 1;
                } else {
                    stack.push(next);
                }
            }
        }

        count.to_string()
    }

    fn part_b(&self, input: &String) -> String {
        let mut map = get_neighbor_map(input);
        let mut visited = HashSet::new();
        let mut path_count_map = HashMap::new();

        // Starting state.
        for (k, v) in map {
            if v.contains("out") {
                *path_count_map.entry(k).or_insert(0) += 1;
            }
        }
        visited.insert("out".to_string());


        loop {
            // Find a candidate.
            let mut candidate_option = None;
            for (k, v) in &map {
                if v.is_subset(&visited) {
                    candidate_option = Some(k.to_string());
                }
            }

            if let Some(candidate) = candidate_option {
                for (k, v) in map {
                    if v.contains(&candidate) {
                        *path_count_map.entry(k).or_insert(0) += path_count_map.get(&candidate).unwrap();
                    }
                }
                visited.insert(candidate);
            } else {
                break;
            }
        }

        count.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_a() -> Result<(), String> {
        Day11 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day11 {}.test_day_part(&'b')
    }
}
