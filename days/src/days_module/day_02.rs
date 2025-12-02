use crate::days_module::day::Day;

pub struct Day02 {}

fn find_invalid_ids(range: &str) -> Vec<usize> {
    println!("{}", range);
    let start_string = range.split('-').nth(0).unwrap();
    let end_string = range.split('-').nth(1).unwrap();

    // Take the first half, first half is shorter for uneven.
    let half_start = start_string[..start_string.len() / 2].parse::<usize>().unwrap_or(start_string.parse::<usize>().unwrap());
    let half_end = end_string[..(end_string.len() + 1) / 2].parse::<usize>().unwrap();
    println!("{} - {}", half_start, half_end);

    // Iterate over the range of half IDs and add valid ones to the returned list.
    let mut invalid_ids: Vec<usize> = Vec::new();
    let start = start_string.parse::<usize>().unwrap();
    let end = end_string.parse::<usize>().unwrap();

    for i in half_start..=half_end {
        let invalid_id = (i.to_string() + i.to_string().as_str()).parse::<usize>().unwrap();
        println!("{}", invalid_id);
        if invalid_id <= end && invalid_id >= start {
            invalid_ids.push(invalid_id);
            println!("ADDED {}", invalid_id);
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
        input.split(',').map(find_invalid_ids).flatten().sum::<usize>().to_string()
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
        Day02 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day02 {}.test_day_part(&'b')
    }
}
