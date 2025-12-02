use crate::days_module::day::Day;
use std::collections::HashSet;

pub struct Day02 {}

fn find_invalid_ids(range: &str) -> Vec<usize> {
    let start = range.split('-').nth(0).unwrap().parse::<usize>().unwrap();
    let end = range.split('-').nth(1).unwrap().parse::<usize>().unwrap();

    let mut invalid_ids: Vec<usize> = Vec::new();
    for candidate in start..=end {
        let candidate_string = &candidate.to_string();
        if candidate_string.len() % 2 == 0 {
            if candidate_string[..candidate_string.len() / 2].to_string()
                == candidate_string[candidate_string.len() / 2..].to_string()
            {
                invalid_ids.push(candidate);
            }
        }
    }

    invalid_ids
}

fn find_all_invalid_ids(range: &str) -> Vec<usize> {
    let start = range.split('-').nth(0).unwrap().parse::<usize>().unwrap();
    let end = range.split('-').nth(1).unwrap().parse::<usize>().unwrap();

    let mut invalid_ids: Vec<usize> = Vec::new();
    for candidate in start..=end {
        let candidate_string = &candidate.to_string();

        for repeat_split in 1..=(candidate_string.len() + 1) / 2 {
            let mut built_string = String::new();

            while built_string.len() < candidate_string.len() {
                built_string += &candidate_string[..repeat_split];
            }

            if &built_string == candidate_string {
                println!("{}", candidate);
                invalid_ids.push(candidate);
            }
        }
    }

    invalid_ids
}

impl Day for Day02 {
    fn get_id(&self) -> String {
        "day_02".to_string()
    }

    fn get_index(&self) -> u8 {
        2
    }

    fn part_a(&self, input: &String) -> String {
        input
            .split(',')
            .map(find_invalid_ids)
            .flatten()
            .sum::<usize>()
            .to_string()
    }

    fn part_b(&self, input: &String) -> String {
        input
            .split(',')
            .map(find_all_invalid_ids)
            .flatten()
            .collect::<HashSet<usize>>()
            .into_iter()
            .sum::<usize>()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_a() -> Result<(), String> {
        Day02 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day02 {}.test_day_part(&'b')
    }
}
