use crate::days_module::day::Day;
use helpers::grid::grid::Grid;
use helpers::grid::grid_index::GridIndex;
use std::collections::HashSet;
use std::str::FromStr;

pub struct Day04 {}

fn find_accessible_indices(grid: &Grid, taken_indices: &HashSet<GridIndex>) -> Vec<GridIndex> {
    let mut accessible_paper_indices = Vec::new();

    for cell in grid.iter() {
        if cell.value != '@' {
            continue;
        }

        let mut count = 0;
        for neighbor in cell.index.moore_neighborhood() {
            match grid.get_cell(&neighbor) {
                Some(c) => {
                    if c.value == '@' && !taken_indices.contains(&c.index) {
                        count += 1;
                    }
                }
                None => continue,
            }
        }

        if count < 4 {
            accessible_paper_indices.push(cell.index.clone());
        }
    }

    accessible_paper_indices
}

impl Day for Day04 {
    fn get_id(&self) -> String {
        "day_04".to_string()
    }

    fn get_index(&self) -> u8 {
        4
    }

    fn part_a(&self, input: &String) -> String {
        let grid = Grid::from_str(&input).unwrap();
        let accessible_paper_indices = find_accessible_indices(&grid, &HashSet::new());
        accessible_paper_indices.len().to_string()
    }

    fn part_b(&self, input: &String) -> String {
        let grid = Grid::from_str(&input).unwrap();
        let mut last_count = usize::MAX;
        let mut taken_indices = HashSet::new();

        loop {
            let accessible_paper_indices = find_accessible_indices(&grid, &taken_indices);
            taken_indices.extend(accessible_paper_indices.iter());

            // Exit if the grid does not evolve.
            if taken_indices.len() == last_count {
                return last_count.to_string();
            }
            last_count = taken_indices.len();
        }
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
