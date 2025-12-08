use std::collections::{HashMap, HashSet, VecDeque};
use helpers::{euclidean_distance, find_numbers};
use crate::days_module::day::Day;

pub struct Day08 {}

pub fn reachable<'a>(
    graph: &'a HashMap<&'a [u32], HashSet<&'a [u32]>>,
    start: &'a [u32],
) -> HashSet<&'a [u32]> {
    let mut visited: HashSet<&[u32]> = HashSet::new();
    let mut queue: VecDeque<&[u32]> = VecDeque::new();

    visited.insert(start);
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        if let Some(neighbors) = graph.get(node) {
            for &next in neighbors {
                if visited.insert(next) {
                    queue.push_back(next);
                }
            }
        }
    }

    visited
}


impl Day for Day08 {
    fn get_id(&self) -> String {
        "day_08".to_string()
    }

    fn get_index(&self) -> u8 {
        8
    }

    fn part_a(&self, input: &String) -> String {
        let numbers: Vec<u32> = find_numbers(input);
        let mut coords = HashSet::new();
        for chunk in numbers.chunks(3) {
            coords.insert(chunk);
        }

        // Calculate edges, sort edge members.
        let mut edges = Vec::new();
        for a in &coords {
            for b in &coords {
                if a != b {
                    let distance = euclidean_distance(a, b);
                    edges.push((distance, a, b));
                }
            }
        }
        edges.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let shortest_edges = edges.iter().take(2000);

        // Build the neighbor map.
        let mut neighbors: HashMap<&[u32], HashSet<&[u32]>> = HashMap::new();
        for edge in shortest_edges {
            neighbors.entry(edge.1).or_default().insert(edge.2);
        }

        // Find the circuit sizes.
        let mut visited_circuits = Vec::new();
        let mut circuit_sizes = Vec::new();
        for node in neighbors.keys() {
            let circuit = reachable(&neighbors, node);

            if visited_circuits.contains(&circuit) {
                continue;
            }

            circuit_sizes.push(circuit.len());
            visited_circuits.push(circuit);
        }

        // Multiply sizes of 3 biggest.
        circuit_sizes.sort();
        circuit_sizes.reverse();
        circuit_sizes.iter().take(3).fold(1, |a, b| a * b).to_string()
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
        Day08 {}.test_day_part(&'a')
    }

    #[test]
    fn test_day_b() -> Result<(), String> {
        Day08 {}.test_day_part(&'b')
    }
}
