use crate::read_input_lines;
use anyhow::Result;
use std::collections::{HashSet, VecDeque};
use itertools::Itertools;

type Point = (u64, u64, u64);

pub fn prepare(file_name: &str) -> Result<Vec<Point>> {
    let input = read_input_lines(file_name)
        .into_iter()
        .map(|line| {
            sscanf!(line, "{u64},{u64},{u64}").ok().unwrap()
        })
        .collect();
    Ok(input)
}

// Answer: 81536
pub fn part_1(input: &Vec<Point>) -> Option<usize> {
    let (answer, _) = connect_circuits(input, 1000).unwrap();
    Some(answer)
}

// Answer: 7017750530
pub fn part_2(input: &Vec<(u64, u64, u64)>) -> Option<usize> {
    let (_, answer) = connect_circuits(input, 0).unwrap();
    Some(answer)
}

#[inline]
pub fn calculate_distance(a:&Point, b:&Point) -> f64 {
    let x = a.0.abs_diff(b.0);
    let y = a.1.abs_diff(b.1);
    let z = a.2.abs_diff(b.2);

    ((x.pow(2) + y.pow(2) + z.pow(2)) as f64).sqrt()
}

pub fn connect_circuits(input: &Vec<Point>, limit:usize) -> Option<(usize, usize)> {

    let mut circuits:VecDeque<HashSet<Point>> = VecDeque::new();
    let mut distances:Vec<(f64, Point, Point)> = Vec::new();
    let mut junctions = input.into_iter();
    let mut last_connection:(Point, Point) = ((0, 0, 0), (0, 0, 0));

    while let Some(p1) = junctions.next() {
        circuits.push_back(HashSet::from([*p1]));
        let tail = junctions.clone();
        for p2 in tail {
            distances.push((calculate_distance(p1, p2), *p1, *p2));
        }
    }
    distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    for (i, (_, p1, p2)) in distances.into_iter().enumerate() {

        if limit > 0 && i == limit { break; }

        let mut circuit:HashSet<Point> = HashSet::from([p1, p2]);
        for _ in 0..circuits.len() {
            let next = circuits.pop_front().unwrap();
            if next.is_disjoint(&circuit) {
                circuits.push_back(next);
            } else {
                circuit = next.union(&circuit).map(|p| *p).collect();
            }
        }
        circuits.push_back(circuit);

        if circuits.len() == 1 {
            last_connection = (p1, p2);
            break;
        }
    }

    let part_1:usize = circuits
        .iter()
        .map(|c| c.len())
        .sorted()
        .rev()
        .take(3)
        .product();

    let part_2 = (last_connection.0.0 * last_connection.1.0) as usize;

    Some((part_1, part_2))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        if let Ok(input) = prepare("day08-example.txt") {
            if let Some((answer, _)) = connect_circuits(&input, 10) {
                assert_eq!(answer, 40)
            }
        }
    }

    #[test]
    fn test_part_2() {
        if let Ok(input) = prepare("day08-example.txt") {
            if let Some((_, answer)) = connect_circuits(&input, 0) {
                assert_eq!(answer, 25272)
            }
        }
    }
}