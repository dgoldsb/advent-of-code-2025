use crate::days_module::day::Day;
use helpers::grid::grid::Grid;
use helpers::grid::grid_index::Direction;
use std::collections::HashSet;
use std::str::FromStr;

pub struct Day07 {}

impl Day for Day07 {
    fn get_id(&self) -> String {
        "day_07".to_string()
    }

    fn get_index(&self) -> u8 {
        7
    }

    fn part_a(&self, input: &String) -> String {
        let grid = Grid::from_str(&input).unwrap();
        let mut stack = vec![grid.find_index(&'S').unwrap()];
        let mut visited = HashSet::new();
        let mut used_splits = HashSet::new();

        while !stack.is_empty() {
            let index = stack.pop().unwrap();

            match grid.move_from_cell(index, &Direction::DOWN) {
                Some(c) => match c.value {
                    '.' => {
                        stack.push(&c.index);
                        visited.insert(&c.index);
                    }
                    '^' => {
                        used_splits.insert(&c.index);
                        for d in vec![Direction::LEFT, Direction::RIGHT] {
                            match grid.move_from_cell(&c.index, &d) {
                                Some(n) => {
                                    if !visited.contains(&n.index) {
                                        stack.push(&n.index);
                                        visited.insert(&n.index);
                                    }
                                }
                                None => {}
                            }
                        }
                    }
                    _ => unreachable!(),
                },
                None => {
                    // Go out of bounds...
                }
            }
        }

        used_splits.len().to_string()
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
        Day07 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day07 {}.test_day_part(&'b')
    }
}
