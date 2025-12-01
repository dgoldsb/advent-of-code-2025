use crate::days_module::day::Day;

pub struct Day01 {}

impl Day for Day01 {
    fn get_id(&self) -> String {
        "day_01".to_string()
    }

    fn get_index(&self) -> u8 {
        1
    }

    fn part_a(&self, input: &String) -> String {
        let mut dial: isize = 50;
        let mut zero_count = 0;

        for line in input.lines() {
            let offset = line[1..].parse::<isize>().unwrap();
            match line.chars().nth(0).unwrap() {
                'L' => {
                    dial -= offset;
                    dial += 100;
                    dial %= 100;
                }
                'R' => {
                    dial += offset;
                    dial %= 100;
                }
                _ => panic!("Unknown direction: {}", line),
            }
            if dial == 0 {
                zero_count += 1;
            }
        }

        zero_count.to_string()
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
        Day01 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day01 {}.test_day_part(&'b')
    }
}
