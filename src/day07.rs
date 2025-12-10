use crate::read_input_lines;
use anyhow::Result;
use std::collections::HashSet;

pub fn prepare(file_name: &str) -> Result<Vec<String>> {
    let input = read_input_lines(file_name);
    Ok(input)
}

pub fn fire_beams(input: &Vec<String>) -> Option<(usize, usize)> {
    let mut split_count = 0;
    let mut paths:Vec<usize> = vec![];

    if let Some(line) = input.first() {
        paths = vec![0; line.len()];
        paths[line.len() / 2] = 1;
    }

    input.into_iter()
        .map(|line| {
            line.chars()
                .enumerate()
                .filter_map(|(x, c)| if c != '.' { Some(x)} else { None })
                .collect::<HashSet<usize>>()
        })
        .filter(|s| !s.is_empty())
        .enumerate()
        .reduce(|(_, a), (by, b)| {
            let splits = a.intersection(&b)
                .map(|aa| {

                    split_count += 1;

                    paths[aa - 1] += paths[*aa];
                    paths[aa + 1] += paths[*aa];
                    paths[*aa] = 0;

                    vec![aa - 1, aa + 1].into_iter()
                })
                .flatten()
                .collect::<HashSet<usize>>();
            let passthrough = a.difference(&b).map(|a| *a ).collect();
            let beams = splits.union(&passthrough).map(|a| *a).collect::<HashSet<usize>>();

            (by, beams)
        });

    let path_count:usize = paths.iter().sum();

    Some((split_count, path_count))
}

//Answer: 1,672
pub fn part_1(input: &Vec<String>) -> Option<usize> {
    if let Some((split_count, _)) = fire_beams(&input) {
        Some(split_count)
    } else {
        None
    }
}

// Answer: 231,229,866,702,355
pub fn part_2(input: &Vec<String>) -> Option<usize> {
    if let Some((_, path_count)) = fire_beams(&input) {
        Some(path_count)
    } else {
        None
    }
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
    fn test_part_2() {
        if let Ok(input) = prepare("day07-example.txt") {
            assert_eq!(part_2(&input), Some(40))
        }
    }
}