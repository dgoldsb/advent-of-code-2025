use std::collections::{HashSet, VecDeque};
use crate::days_module::day::Day;
use regex::Regex;

pub struct Day10 {}

fn parse_line_regex(input: &str) -> (Vec<bool>, Vec<HashSet<usize>>, Vec<usize>) {
    let re_pattern = Regex::new(r"\[(.*?)\]").unwrap();
    let re_groups = Regex::new(r"\((.*?)\)").unwrap();
    let re_set = Regex::new(r"\{(.*?)\}").unwrap();

    // Parse the desired light pattern.
    let pattern_caps = re_pattern.captures(input).expect("Missing [...] block");

    let pattern_str = &pattern_caps[1]; // inside [...]
    let pattern: Vec<bool> = pattern_str
        .chars()
        .map(|c| match c {
            '.' => false,
            '#' => true,
            _ => panic!("Invalid character in pattern"),
        })
        .collect();

    // Parse all switch effects.
    let mut groups = Vec::new();
    for caps in re_groups.captures_iter(input) {
        let inner = caps[1].trim();

        let nums = inner
            .split(',')
            .map(|s| s.trim().parse::<usize>().unwrap())
            .collect::<HashSet<_>>();

        groups.push(nums);
    }

    // Parse the joltage.
    let set_caps = re_set.captures(input).expect("Missing {...} block");
    let set_str = set_caps[1].trim();

    let set = if set_str.is_empty() {
        vec![]
    } else {
        set_str
            .split(',')
            .map(|s| s.trim().parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    };

    (pattern, groups, set)
}

fn solve_a(target: &Vec<bool>, buttons: &Vec<HashSet<usize>>) -> usize {
    // TODO: Skip visited set.
    let first_state = target.into_iter().map(|_| false).collect::<Vec<bool>>();
    let mut queue = VecDeque::new();
    queue.push_back((first_state, 0));

    while queue.len() > 0 {
        let (state, count) = queue.pop_front().unwrap();

        if target == &state {
            return count;
        }

        for button in buttons {
            let mut iter_state = Vec::new();
            for (i, b) in state.clone().into_iter().enumerate() {
                 if button.contains(&i) {
                     iter_state.push(!b);
                 } else {
                     iter_state.push(b);
                 }
                queue.push_back((iter_state.clone(), count + 1));
            }
        }
    }

    unreachable!();
}

impl Day for Day10 {
    fn get_id(&self) -> String {
        "day_10".to_string()
    }

    fn get_index(&self) -> u8 {
        10
    }

    fn part_a(&self, input: &String) -> String {
        input
            .lines()
            .map(parse_line_regex)
            .map(|(pattern, groups, _)| solve_a(&pattern, &groups))
            .sum::<usize>()
            .to_string()
    }

    fn part_b(&self, input: &String) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_a() -> Result<(), String> {
        Day10 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day10 {}.test_day_part(&'b')
    }
}
