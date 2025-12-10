use crate::read_input_lines;
use anyhow::Result;
use itertools::Itertools;
use std::collections::HashSet;

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

pub fn calc(a:&Point, b:&Point) -> f64 {
    let x = a.0.abs_diff(b.0);
    let y = a.1.abs_diff(b.1);
    let z = a.2.abs_diff(b.2);

    ((x.pow(2) + y.pow(2) + z.pow(2)) as f64).sqrt()
}

pub fn part_1(input: &Vec<Point>) -> Option<usize> {

    let mut dist:Vec<(f64, Point, Point)> = Vec::new();
    let mut points = input.into_iter();
    while let Some(p1) = points.next() {
        let tail = points.clone();
        for p2 in tail {
            dist.push((calc(p1, p2), *p1, *p2));
        }
    }

    dist.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

    let mut circuits:Vec<HashSet<Point>> = Vec::new();
    for (_, p1, p2) in dist {
        if let Some(idx) = circuits
            .clone()
            .into_iter()
            .filter(|c| {
                !(c.contains(&p1) && c.contains(&p2))
            })
            .position(|c| {
                c.contains(&p1) || c.contains(&p2)
            })
        {
            circuits[idx].insert(p1);
            circuits[idx].insert(p2);
        } else {
            circuits.push(vec![p1, p2].into_iter().collect());
        }
    }

    circuits.sort_by(|a, b| a.len().partial_cmp(&b.len()).unwrap());

    // print!("{:#?}", circuits);

    Some(0)
}

pub fn part_2(_input: &Vec<(u64, u64, u64)>) -> Option<usize> {
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        if let Ok(input) = prepare("day08-example.txt") {

            let answer = part_1(&input).unwrap();

            println!("\n----> {answer}\n");

            // assert_eq!(part_1(&input), Some(40))
        }
    }

    #[test]
    #[ignore]
    fn test_part_2() {
        if let Ok(input) = prepare("day08-example.txt") {
            assert_eq!(part_2(&input), Some(1))
        }
    }
}