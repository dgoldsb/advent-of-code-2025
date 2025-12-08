use crate::days_module::day::Day;
use helpers::{euclidean_distance, find_numbers};
use helpers::union_find::union_find::UnionFind;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;

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

fn solve(numbers: Vec<u32>, count: usize) -> Vec<usize> {
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
    let shortest_edges = edges.iter().take(count);

    // Build the neighbor map.
    let mut neighbors: HashMap<&[u32], HashSet<&[u32]>> = HashMap::new();
    for edge in shortest_edges {
        neighbors.entry(edge.1).or_default().insert(edge.2);
    }

    // Find the circuit sizes.
    let mut visited_circuits = Vec::new();
    for node in neighbors.keys() {
        let circuit = reachable(&neighbors, node);

        if visited_circuits.contains(&circuit) {
            continue;
        }

        visited_circuits.push(circuit);
    }

    visited_circuits.iter().map(|c| c.len()).collect::<Vec<_>>()
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
        let mut circuit_sizes = solve(numbers, 2000);

        // Multiply sizes of 3 biggest.
        circuit_sizes.sort();
        circuit_sizes.reverse();
        circuit_sizes
            .iter()
            .take(3)
            .fold(1, |a, b| a * b)
            .to_string()
    }

    fn part_b(&self, input: &String) -> String {
        let numbers: Vec<u32> = find_numbers(input);

        // TODO: Some duplicate code.
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
        // TODO: End duplicate code.

        let mut union_find = UnionFind::new();
        for edge in edges {
            union_find.union(edge.1, edge.2);

            println!("{:?}", union_find.size.iter().max().unwrap());
            if *union_find.size.iter().max().unwrap() == (numbers.len() / 3) {
                println!("{}", (edge.1[0] * edge.2[0]).to_string());

                // 2639735259 too low
                panic!();
            }
        }

        unreachable!()
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
