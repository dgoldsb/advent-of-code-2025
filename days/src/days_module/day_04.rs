use crate::days_module::day::Day;
use helpers::grid::grid::Grid;
use std::str::FromStr;

pub struct Day04 {}

impl Day for Day04 {
    fn get_id(&self) -> String {
        "day_04".to_string()
    }

    fn get_index(&self) -> u8 {
        4
    }

    fn part_a(&self, input: &String) -> String {
        let grid = Grid::from_str(&input).unwrap();
        let mut accessible_paper_count = 0;

        for cell in grid.iter() {
            if cell.value != '@' {
                continue;
            }

            let mut count = 0;
            for neighbor in cell.index.moore_neighborhood() {
                match grid.get_cell(&neighbor) {
                    Some(c) => {
                        if c.value == '@' {
                            count += 1;
                        }
                    }
                    None => continue,
                }
            }

            if count < 4 {
                accessible_paper_count += 1;
            }
        }
        accessible_paper_count.to_string()
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
        Day04 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day04 {}.test_day_part(&'b')
    }
}
