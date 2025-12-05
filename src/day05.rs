use crate::read_input_lines;
use anyhow::Result;
use itertools::Itertools;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum RangeBound {
    Start,
    End
}
use RangeBound::*;

pub fn prepare(file_name: &str) -> Result<(Vec<(u64, u64)>, Vec<u64>)> {
    let input = read_input_lines(file_name);
    let mut range_bounds:Vec<(u64, RangeBound)> = Vec::new();
    let mut ids:Vec<u64> = Vec::new();
    for line in input.into_iter() {
        if let Ok((start, end)) = sscanf!(line, "{u64}-{u64}") {
            range_bounds.push((start, Start));
            range_bounds.push((end, End));
        } else {
            if let Ok(id) = sscanf!(line, "{u64}") {
                ids.push(id);
            }
        }
    }

    let mut ranges:Vec<(u64, u64)> = Vec::new();
    let mut start_stack:Vec<(u64, RangeBound)> = Vec::new();
    for bound in range_bounds.into_iter().sorted() {
        match bound {
            (_, Start) => start_stack.push(bound),
            (_, End) => {
                if let Some(start) = start_stack.pop() {
                    if start_stack.is_empty() {
                        ranges.push((start.0, bound.0))
                    }
                }
            },
        }
    }
    ranges.sort();

    Ok((ranges, ids))
}

// Answer: 862
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

// Answer: 357907198933892
pub fn part_2(input: &(Vec<(u64, u64)>, Vec<u64>)) -> Option<usize> {
    let (ranges, _) = input;
    let total = ranges
        .iter()
        .fold(0u64 , |total, (start, end)| {
            total + end - start + 1
        });
    Some(total as usize)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_prepare() {
        if let Ok(input) = prepare("day05-example.txt") {
            let (ranges, ids) = input;
            assert_eq!(ranges.first(), Some(&(3, 5)));
            assert_eq!(ranges.last(), Some(&(10, 20)));
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
    fn test_part_2() {
        if let Ok(input) = prepare("day05-example.txt") {
            assert_eq!(part_2(&input), Some(14))
        }
    }
}