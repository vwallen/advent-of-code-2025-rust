use crate::read_input_lines;
use anyhow::Result;

pub fn prepare(file_name: &str) -> Result<Vec<String>> {
    let input = read_input_lines(file_name);
    Ok(input)
}

pub fn find_highest(batteries:&String) -> u64 {
    let batteries: Vec<u64> = batteries
        .chars()
        .map(|c| c.to_digit(10).map(u64::from).unwrap())
        .collect();
    let mut digits = vec![0u64; 2];
    for (i, d) in batteries.iter().enumerate() {
        if d > &digits[0] && i < batteries.len() - 1 {
            digits[0] = *d;
            digits[1] = 0;
        } else if d > &digits[1] {
            digits[1] = *d;
        }
    }
    digits[0] * 10 + digits[1]
}


pub fn find_highest_12(batteries:&String) -> u64 {
    let batteries: Vec<u64> = batteries
        .chars()
        .map(|c| c.to_digit(10).map(u64::from).unwrap())
        .collect();
    let mut digits = vec![0u64; 12];
    for (i, d) in batteries.iter().enumerate() {
        if d > &digits[0] && i < batteries.len() - 11 {
            digits[0] = *d;
            digits[1] = 0;
            digits[2] = 0;
            digits[3] = 0;
            digits[4] = 0;
            digits[5] = 0;
            digits[6] = 0;
            digits[7] = 0;
            digits[8] = 0;
            digits[9] = 0;
            digits[10] = 0;
            digits[11] = 0;
        } else if d > &digits[1] && i < batteries.len() - 10 {
            digits[1] = *d;
            digits[2] = 0;
            digits[3] = 0;
            digits[4] = 0;
            digits[5] = 0;
            digits[6] = 0;
            digits[7] = 0;
            digits[8] = 0;
            digits[9] = 0;
            digits[10] = 0;
            digits[11] = 0;
        } else if d > &digits[2] && i < batteries.len() - 9 {
            digits[2] = *d;
            digits[3] = 0;
            digits[4] = 0;
            digits[5] = 0;
            digits[6] = 0;
            digits[7] = 0;
            digits[8] = 0;
            digits[9] = 0;
            digits[10] = 0;
            digits[11] = 0;
        } else if d > &digits[3] && i < batteries.len() - 8 {
            digits[3] = *d;
            digits[4] = 0;
            digits[5] = 0;
            digits[6] = 0;
            digits[7] = 0;
            digits[8] = 0;
            digits[9] = 0;
            digits[10] = 0;
            digits[11] = 0;
        } else if d > &digits[4] && i < batteries.len() - 7 {
            digits[4] = *d;
            digits[5] = 0;
            digits[6] = 0;
            digits[7] = 0;
            digits[8] = 0;
            digits[9] = 0;
            digits[10] = 0;
            digits[11] = 0;
        } else if d > &digits[5] && i < batteries.len() - 6 {
            digits[5] = *d;
            digits[6] = 0;
            digits[7] = 0;
            digits[8] = 0;
            digits[9] = 0;
            digits[10] = 0;
            digits[11] = 0;
        } else if d > &digits[6] && i < batteries.len() - 5 {
            digits[6] = *d;
            digits[7] = 0;
            digits[8] = 0;
            digits[9] = 0;
            digits[10] = 0;
            digits[11] = 0;
        } else if d > &digits[7] && i < batteries.len() - 4 {
            digits[7] = *d;
            digits[8] = 0;
            digits[9] = 0;
            digits[10] = 0;
            digits[11] = 0;
        } else if d > &digits[8] && i < batteries.len() - 3 {
            digits[8] = *d;
            digits[9] = 0;
            digits[10] = 0;
            digits[11] = 0;
        } else if d > &digits[9] && i < batteries.len() - 2 {
            digits[9] = *d;
            digits[10] = 0;
            digits[11] = 0;
        } else if d > &digits[10] && i < batteries.len() - 1 {
            digits[10] = *d;
            digits[11] = 0;
        } else if d > &digits[11] && i < batteries.len() - 0 {
            digits[11] = *d;
        }
    }
    digits
        .iter()
        .rev()
        .enumerate()
        .map(|(i, d)| 10u64.pow(i as u32) * d)
        .sum()
}


pub fn part_1(input: &Vec<String>) -> Option<u64> {
    let total:u64 = input.iter().map(find_highest).sum();
    Some(total)
}

pub fn part_2(input: &Vec<String>) -> Option<u64> {
    let total:u64 = input.iter().map(find_highest_12).sum();
    Some(total)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_find_highest() {
        assert_eq!(find_highest(&"12345".into()), 45);
        assert_eq!(find_highest(&"12151".into()), 51);
        assert_eq!(find_highest(&"35215".into()), 55);
        assert_eq!(find_highest(&"12453".into()), 53);
        assert_eq!(find_highest(&"45321".into()), 53);
    }

    #[test]
    fn test_part_1() {
        if let Ok(input) = prepare("day03-example.txt") {
            assert_eq!(part_1(&input), Some(357))
        }
    }

    #[test]
    fn test_part_2() {
        if let Ok(input) = prepare("day03-example.txt") {
            assert_eq!(part_2(&input), Some(3121910778619))
        }
    }
}