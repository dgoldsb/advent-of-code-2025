use crate::days_module::day::Day;
use lazy_static::lazy_static;

use helpers::{find_numbers, transpose_string};
use regex::Regex;

pub fn find_operators(s: &str) -> Vec<char> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\+|\*").unwrap();
    }

    RE.find_iter(s)
        .map(|m| m.as_str().chars().next().unwrap())
        .collect()
}

pub struct Day06 {}

impl Day for Day06 {
    fn get_id(&self) -> String {
        "day_06".to_string()
    }

    fn get_index(&self) -> u8 {
        6
    }

    fn part_a(&self, input: &String) -> String {
        let numbers = find_numbers(input);
        let operators = find_operators(input);
        let equation_count = operators.len();

        let mut answers: Vec<usize> = operators
            .clone()
            .into_iter()
            .map(|x| if x == '*' { 1 } else { 0 })
            .collect();

        for (index, number) in numbers.iter().enumerate() {
            match operators[index % equation_count] {
                '+' => answers[index % equation_count] += number,
                '*' => answers[index % equation_count] *= number,
                _ => unreachable!(),
            }
        }

        answers.into_iter().sum::<usize>().to_string()
    }

    fn part_b(&self, input: &String) -> String {
        let regex = Regex::new(r"\n\s*\n").unwrap();
        let raw_transposed_input = transpose_string(input);
        let transposed_input = regex.replace_all(&raw_transposed_input, "\n\n\n");
        let mut total = 0;

        for equation in transposed_input.split("\n\n\n") {
            let numbers: Vec<usize> = find_numbers(&equation);
            let operator = find_operators(&equation)[0];
            let mut answer = match operator {
                '+' => 0,
                '*' => 1,
                _ => unreachable!(),
            };
            for number in numbers {
                match operator {
                    '+' => answer += number,
                    '*' => answer *= number,
                    _ => unreachable!(),
                }
            }

            total += answer;
        }

        total.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_a() -> Result<(), String> {
        Day06 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day06 {}.test_day_part(&'b')
    }
}
