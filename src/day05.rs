use crate::read_input_lines;
use anyhow::Result;
// use itertools::Itertools;

pub fn prepare(file_name: &str) -> Result<(Vec<(u64, u64)>, Vec<u64>)> {
    let input = read_input_lines(file_name);
    let mut ranges:Vec<(u64, u64)> = Vec::new();
    let mut ids:Vec<u64> = Vec::new();
    for line in input.into_iter() {
        if let Ok((start, end)) = sscanf!(line, "{u64}-{u64}") {
            ranges.push((start, end));
        } else {
            if let Ok(id) = sscanf!(line, "{u64}") {
                ids.push(id);
            }
        }
    }
    ranges.sort();
    // println!("{:?}", ranges);
    // println!("{:?}", ids);
    Ok((ranges, ids))
}

pub fn part_1(input: &(Vec<(u64, u64)>, Vec<u64>)) -> Option<usize> {
    let (ranges, ids) = input;
    let count:usize = ids
        .iter()
        .fold(0 , |count, id| {
           if ranges.iter().any(|(s, e)| s <= id && id <= e) {
               count + 1
           } else {
               count
           }
        });

    Some(count)
}

pub fn part_2(_input: &(Vec<(u64, u64)>, Vec<u64>)) -> Option<usize> {
    None
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_prepare() {
        if let Ok(input) = prepare("day05-example.txt") {
            let (ranges, ids) = input;
            assert_eq!(ranges.first(), Some(&(3, 5)));
            assert_eq!(ranges.last(), Some(&(16, 20)));
            assert_eq!(ids.first(), Some(&1));
            assert_eq!(ids.last(), Some(&32));
        }
    }

    #[test]
    fn test_part_1() {
        if let Ok(input) = prepare("day05-example.txt") {
            assert_eq!(part_1(&input), Some(3))
        }
    }

    #[test]
    #[ignore]
    fn test_part_2() {
        if let Ok(input) = prepare("day05-example.txt") {
            assert_eq!(part_2(&input), Some(1))
        }
    }
}