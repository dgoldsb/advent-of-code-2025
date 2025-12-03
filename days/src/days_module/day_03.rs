use crate::days_module::day::Day;

fn find_highest_joltage(input: &str, digit_count: usize) -> usize {
    let chars: Vec<char> = input.chars().collect();
    let mut digits: Vec<char> = Vec::new();
    let mut digits_left = digit_count;
    let mut start_index = 0;

    for _ in 0..digit_count {
        digits_left -= 1;
        let mut highest_digit = '/';  // cheeky, ASCII value is one below '0'
        let mut digit_index = 0;

        for i in start_index..chars.len() - digits_left {
            if highest_digit < chars[i] {
                highest_digit = chars[i];
                digit_index = i;
            }
        }

        digits.push(chars[digit_index]);
        start_index = digit_index + 1;
    }

    digits.into_iter().collect::<String>().parse::<usize>().unwrap()
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
            .map(|line| find_highest_joltage(line, 2))
            .sum::<usize>()
            .to_string()
    }

    fn part_b(&self, input: &String) -> String {
        input
            .lines()
            .map(|line| find_highest_joltage(line, 12))
            .sum::<usize>()
            .to_string()
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
