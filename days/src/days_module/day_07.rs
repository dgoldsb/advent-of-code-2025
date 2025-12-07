use crate::days_module::day::Day;
use helpers::grid::grid::Grid;
use helpers::grid::grid_index::{Direction, GridIndex};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
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
        let grid = Grid::from_str(&input).unwrap();
        let mut heap = BinaryHeap::new();
        let mut weights: HashMap<&GridIndex, u128> = HashMap::new();
        let mut total_paths = 0;

        let start_index = grid.find_index(&'S').unwrap();
        heap.push((-1 * start_index.x, start_index));
        weights.insert(start_index, 1);

        while !heap.is_empty() {
            let tuple = heap.pop().unwrap();
            let weight = weights.get(&tuple.1).unwrap().clone();

            match grid.move_from_cell(tuple.1, &Direction::DOWN) {
                Some(c) => match c.value {
                    '.' => {
                        // Update the weight.
                        let existing_weight = weights.get(&c.index).unwrap_or(&0).clone();
                        weights.insert(&c.index, existing_weight + weight);

                        if existing_weight == 0 {
                            heap.push((-1 * c.index.x, &c.index));
                        }
                    }
                    '^' => {
                        for d in vec![Direction::LEFT, Direction::RIGHT] {
                            match grid.move_from_cell(&c.index, &d) {
                                Some(n) => {
                                    // Update the weight.
                                    let existing_weight =
                                        weights.get(&n.index).unwrap_or(&0).clone();
                                    weights.insert(&n.index, existing_weight + weight);

                                    if existing_weight == 0 {
                                        heap.push((-1 * n.index.x, &n.index));
                                    }
                                }
                                None => unreachable!(),
                            }
                        }
                    }
                    _ => unreachable!(),
                },
                None => {
                    total_paths += weight;
                }
            }
        }

        total_paths.to_string()
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
