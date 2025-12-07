use std::collections::HashSet;
use crate::read_input_lines;
use anyhow::Result;

pub fn prepare(file_name: &str) -> Result<Vec<String>> {
    let input = read_input_lines(file_name);
    Ok(input)
}

//Answer: 1672
pub fn part_1(input: &Vec<String>) -> Option<usize> {
    let mut split_count = 1;
    let _beams:HashSet<usize> = input.into_iter()
        .map(|line| {
            line.chars()
                .enumerate()
                .filter_map(|(x, c)| if c != '.' { Some(x)} else { None })
                .collect::<HashSet<usize>>()
        })
        .filter(|s| !s.is_empty())
        .reduce(|a, b| {
            let splits = a.intersection(&b)
                .map(|aa| {
                    split_count += 1;
                    vec![aa - 1, aa + 1].into_iter()
                })
                .flatten()
                .collect::<HashSet<usize>>();
            let passthrough = a.difference(&b).map(|a| *a ).collect();
            let beams = splits.union(&passthrough).map(|a| *a).collect::<HashSet<usize>>();
            beams
        }).unwrap();

    split_count -= 1;

    Some(split_count)
}

pub fn part_2(_input: &Vec<String>) -> Option<usize> {
    None
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_part_1() {
        if let Ok(input) = prepare("day07-example.txt") {
            assert_eq!(part_1(&input), Some(21))
        }
    }

    #[test]
    #[ignore]
    fn test_part_2() {
        if let Ok(input) = prepare("day07-example.txt") {
            assert_eq!(part_2(&input), Some(1))
        }
    }
}