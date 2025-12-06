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
        line.split_ascii_whitespace()
            .enumerate()
            .map(|(idx, c)| match c {
                "*" => Product(factors[idx].clone()),
                "+" => Sum(factors[idx].clone()),
                 _  => {
                     println!("Found {}", c);
                     unreachable!()
                 },
            })
            .collect()
    }).expect("Unable to process operations");

    Ok(ops)
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

// Answer: ????
pub fn part_2(_input: &Vec<String>) -> Option<usize> {
    None
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
    #[ignore]
    fn test_part_2() {
        if let Ok(input) = prepare("day06-example.txt") {
            assert_eq!(part_2(&input), Some(1))
        }
    }
}