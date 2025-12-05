use crate::days_module::day::Day;

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
        "".to_string()
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
