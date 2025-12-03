use crate::days_module::day::Day;

fn find_highest_joltage(input: &str) -> usize {
    let chars: Vec<char> = input.chars().collect();

    // Find the highest first digit.
    let mut highest_first_digit = '/';
    let mut first_digit_index = 0;
    for i in 0..chars.len() - 1 {
        if highest_first_digit < chars[i] {
            highest_first_digit = chars[i];
            first_digit_index = i;
        }
    }

    // find the highest second digit.
    let mut highest_second_digit = '/';
    for i in first_digit_index + 1..chars.len() {
        if highest_second_digit < chars[i] {
            highest_second_digit = chars[i];
        }
    }

    (highest_first_digit.to_string() + &*highest_second_digit.to_string())
        .parse::<usize>()
        .unwrap()
}

pub struct Day03 {}

impl Day for Day03 {
    fn get_id(&self) -> String {
        "day_03".to_string()
    }

    fn get_index(&self) -> u8 {
        3
    }

    fn part_a(&self, input: &String) -> String {
        input
            .lines()
            .map(find_highest_joltage)
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
        Day03 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day03 {}.test_day_part(&'b')
    }
}
