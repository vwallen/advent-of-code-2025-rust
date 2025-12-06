use std::cmp::max;
use crate::read_input_lines;
use anyhow::Result;

#[derive(Debug)]
pub enum Op {
    Product(Vec<u64>),
    Sum(Vec<u64>),
}
use Op::*;

pub fn prepare(file_name: &str) -> Result<Vec<String>> {
    let input = read_input_lines(file_name);
    Ok(input)
}

pub fn prepare_part_1(input:&Vec<String>) -> Result<Vec<Op>> {
    let mut factors:Vec<Vec<u64>> = Vec::new();

    for line in input.iter() {
        for (idx, token) in line.split_ascii_whitespace().enumerate() {
            if let Ok(value) = token.parse::<u64>() {
                if let Some(values) = factors.get_mut(idx) {
                    values.push(value);
                } else {
                    factors.push(vec![value]);
                }
            }
        }
    }

    let ops:Vec<Op> = input.last().map(|line| {
        factors_to_ops(line, &factors)
    }).expect("Unable to process operations");

    Ok(ops)
}

pub fn prepare_part_2(input:&Vec<String>) -> Result<Vec<Op>> {

    let grid:Vec<Vec<char>> = input
        .iter()
        .map(|line| {
            line.chars().collect()
        })
        .collect();
    let h = grid.len();
    let w = grid.iter().fold(0, |len, line| max(len, line.len()));

    let mut rotated = vec![vec![' '; h]; w];
    grid[..(h - 1)].into_iter().enumerate().for_each(|(y, row)| {
        row.into_iter().enumerate().for_each(|(x, c)| {
            rotated[x][y] = *c;
        })
    });

    let factors:Vec<_> = rotated
        .into_iter()
        .map(|row| row.iter().collect::<String>().clone().trim().to_string())
        .collect::<Vec<String>>()
        .as_slice()
        .split(String::is_empty)
        .map(|factors| {
            factors.iter().map(|word| word.parse::<u64>().unwrap()).collect::<Vec<u64>>()
        })
        .collect();

    let ops:Vec<Op> = input.last().map(|line| {
        factors_to_ops(line, &factors)
    }).expect("Unable to process operations");

    Ok(ops)
}

pub fn factors_to_ops(line:&String, factors:&Vec<Vec<u64>>) -> Vec<Op> {
    line.split_ascii_whitespace()
        .enumerate()
        .map(|(idx, c)| match c {
            "*" => Product(factors[idx].clone()),
            "+" => Sum(factors[idx].clone()),
            _  => unreachable!()
        })
        .collect()
}

pub fn calc(op:&Op) -> u64 {
    match op {
        Product(factors) => factors.iter().product(),
        Sum(factors) => factors.iter().sum()
    }
}

// Answer: 4405895212738
pub fn part_1(input: &Vec<String>) -> Option<usize> {
    let ops = prepare_part_1(input).unwrap();
    let total = ops.iter().map(calc).sum::<u64>();
    Some(total as usize)
}

// Answer: 7450962489289
pub fn part_2(input: &Vec<String>) -> Option<usize> {
    let ops = prepare_part_2(input).unwrap();
    let total = ops.iter().map(calc).sum::<u64>();
    Some(total as usize)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_part_1() {
        if let Ok(input) = prepare("day06-example.txt") {
            assert_eq!(part_1(&input), Some(4277556))
        }
    }

    #[test]
    fn test_part_2() {
        if let Ok(input) = prepare("day06-example.txt") {
            assert_eq!(part_2(&input), Some(3263827))
        }
    }
}