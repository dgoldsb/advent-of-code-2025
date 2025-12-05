use crate::days_module::day::Day;
use std::cmp::min;
use std::collections::VecDeque;

pub struct Day05 {}

impl Day for Day05 {
    fn get_id(&self) -> String {
        "day_05".to_string()
    }

    fn get_index(&self) -> u8 {
        5
    }

    fn part_a(&self, input: &String) -> String {
        let split = input.split_once("\n\n").unwrap();

        // Find the ranges.
        let mut ranges = Vec::new();
        for line in split.0.lines() {
            let split_line = line.split_once("-").unwrap();
            ranges.push(
                split_line.0.parse::<usize>().unwrap()..=split_line.1.parse::<usize>().unwrap(),
            );
        }

        // Count the fresh ingredients.
        let mut count = 0;
        for line in split.1.lines() {
            let sku = line.parse::<usize>().unwrap();
            for range in ranges.iter() {
                if range.contains(&sku) {
                    count += 1;
                    break;
                }
            }
        }

        count.to_string()
    }

    fn part_b(&self, input: &String) -> String {
        let split = input.split_once("\n\n").unwrap();

        // Find the ranges.
        let mut ranges = Vec::new();
        for line in split.0.lines() {
            let split_line = line.split_once("-").unwrap();
            ranges.push((
                split_line.0.parse::<usize>().unwrap(),
                split_line.1.parse::<usize>().unwrap(),
            ));
        }

        ranges.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        // Merge the ranges.
        let mut total: u128 = 0;
        let mut stack = ranges
            .iter()
            .map(|t| t.clone())
            .collect::<Vec<(usize, usize)>>();
        while !stack.is_empty() {
            if stack.len() == 1 {
                let item = stack.pop().unwrap();
                total += (item.1 - item.0 + 1) as u128;
                break;
            }

            let last = stack.pop().unwrap();
            let second_last = stack.pop().unwrap();
            if last.0 <= second_last.1 || (second_last.1 + 1 == last.0) {
                let new_item = (min(last.0, second_last.0), last.1);
                stack.push(new_item);
            } else {
                stack.push(second_last);
                total += (last.1 - last.0 + 1) as u128;
            }
        }

        total.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_a() -> Result<(), String> {
        Day05 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day05 {}.test_day_part(&'b')
    }
}
