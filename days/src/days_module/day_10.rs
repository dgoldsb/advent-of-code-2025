use crate::days_module::day::Day;
use regex::Regex;

pub struct Day10 {}

fn solve_line(line: &str) -> usize {
    let start_state_regex = Regex::new(r"\[[#\.]+\]");
    0
}

impl Day for Day10 {
    fn get_id(&self) -> String {
        "day_010".to_string()
    }

    fn get_index(&self) -> u8 {
        10
    }

    fn part_a(&self, input: &String) -> String {
        input.lines().map(solve_line).sum::<usize>().to_string()
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
