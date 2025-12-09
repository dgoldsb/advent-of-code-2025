use std::cmp::{max, min};
use helpers::cube::cube::Cube;
use helpers::find_numbers;
use crate::days_module::day::Day;

pub struct Day09 {}

// Initial strategy: finding squares that have no intersecting edges with the outside...
// This fails one assumption,it appears possible edges go through the square.
// My answer 60737536 was too low.

fn calculate_size(a: &[isize], b: &[isize]) -> isize {
    let x_difference = (a[0] - b[0]).abs() + 1;
    let y_difference = (a[1] - b[1]).abs() + 1;
    x_difference * y_difference
}

fn edge_intersects_square(s: &[isize], e: &[isize], a: &[isize], b: &[isize]) -> bool {
    let square = Cube::new(
        (min(a[0], b[0]) + 1)..=(max(a[0], b[0]) - 1),
        (min(a[1], b[1]) + 1)..=(max(a[1], b[1]) - 1),
        0..=0,
    );
    let edge = Cube::new(
        min(s[0], e[0])..=max(s[0], e[0]),
        min(s[1], e[1])..=max(s[1], e[1]),
        0..=0,
    );

    edge.intersects(&square)
}

impl Day for Day09 {
    fn get_id(&self) -> String {
        "day_09".to_string()
    }

    fn get_index(&self) -> u8 {
        9
    }

    fn part_a(&self, input: &String) -> String {
        let numbers: Vec<isize> = find_numbers(input);
        let mut coords = Vec::new();
        for chunk in numbers.chunks(2) {
            coords.push(chunk);
        }

        let mut biggest = 0;
        for a in &coords {
            for b in &coords {
                let size = calculate_size(a, b);
                if size > biggest {
                    biggest = size;
                }
            }
        }
        biggest.to_string()
    }

    fn part_b(&self, input: &String) -> String {
        let numbers: Vec<isize> = find_numbers(input);
        let mut coords = Vec::new();
        for chunk in numbers.chunks(2) {
            coords.push(chunk);
        }
        let mut edges_coords = coords.clone();
        edges_coords.push(coords[0]);


        let mut biggest = 0;
        for a in &coords {
            for b in &coords {
                // Walk the path, if it intersects anything but the border it is invalid.
                let mut intersects = false;
                for i in 0..coords.len() {
                    if edge_intersects_square(edges_coords[i], edges_coords[i + 1], a, b) {
                        intersects = true;
                        break;
                    }
                }

                // TODO: If it is external, it is invalid.

                if intersects {
                    break;
                }

                // If it is valid, calculate and store the size.
                let size = calculate_size(a, b);
                if size > biggest {
                    println!("{}", size);
                    biggest = size;
                }
            }
        }

        assert!(biggest > 60737536);
        biggest.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_a() -> Result<(), String> {
        Day09 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day09 {}.test_day_part(&'b')
    }

    #[test]
    fn test_intersection() {
        let result = edge_intersects_square(&[7, 1], &[7, 3], &[2, 3], &[11, 1]);
        assert_eq!(result, true)
    }
}
