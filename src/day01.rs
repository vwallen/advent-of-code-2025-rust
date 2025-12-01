use crate::read_input_lines;
use anyhow::Result;

static DIAL_SIZE:i64 = 100;

#[derive(Debug)]
pub enum Op {
    Left(i64),
    Right(i64)
}

pub fn prepare(file_name: &str) -> Result<Vec<Op>> {
    let input = read_input_lines(file_name);
    let ops = input.iter().map(|line| {
        let (dir, offset) = sscanf!(line, "{char}{i64}").ok().unwrap();
        match dir {
            'L' => Op::Left(offset),
            'R' => Op::Right(offset),
            _   => panic!("Invalid command: {}", line)
        }
    }).collect();
    Ok(ops)
}

pub fn part_1(input: &Vec<Op>) -> Option<usize> {
    let (_pos, count) = input
        .iter()
        .fold((50, 0usize), |(pos, count), op| {
            let pos = match op {
                Op::Left(offset)  => pos - offset,
                Op::Right(offset) => pos + offset
            }.rem_euclid(DIAL_SIZE);
            if pos == 0 { (pos, count + 1) } else { (pos, count) }
        });
    Some(count)
}

pub fn part_2(input: &Vec<Op>) -> Option<usize> {
    let (_pos, count) = input
        .iter()
        .fold((50, 0i64), |(pos, count), op| {
            let (pos, zeroes) = match op {
                Op::Left(offset)  => {
                    let new_pos = (pos - offset).rem_euclid(DIAL_SIZE);
                    let mut zeroes = ((pos - offset) / DIAL_SIZE).abs();
                    if pos == *offset {
                        zeroes = 1;
                    } else {
                        if pos < *offset { zeroes += 1 };
                        if pos == 0 { zeroes -= 1 };
                    }
                    (new_pos, zeroes)
                },
                Op::Right(offset) => (
                    (pos + offset).rem_euclid(DIAL_SIZE),
                    (pos + offset) / DIAL_SIZE
                )
            };
            let count = count + zeroes;
            (pos, count)
        });
    Some(count as usize)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_part_1() {
        if let Ok(input) = prepare("day01-example.txt") {
            assert_eq!(part_1(&input), Some(3))
        }
    }

    #[test]
    fn test_part_2() {
        if let Ok(input) = prepare("day01-example.txt") {
            assert_eq!(part_2(&input), Some(6))
        }
    }
}