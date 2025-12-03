use crate::read_input_lines;
use anyhow::Result;

pub fn prepare(file_name: &str) -> Result<Vec<Vec<u64>>> {
    let input = read_input_lines(file_name)
        .iter()
        .map(|line| {
            line
                .chars()
                .map(|c| c.to_digit(10).map(u64::from).unwrap())
                .collect()
        })
        .collect();
    Ok(input)
}

pub fn find_highest_n(n:usize, batteries:&Vec<u64>) -> u64 {
    let mut digits = vec![0u64; n];
    let len = batteries.len();
    for (i, d) in batteries.iter().enumerate() {
        for j in 0..n {
            if d > &digits[j] && i < len - (n - (j + 1)) {
                digits[j] = *d;
                if j + 1 < n {
                   digits[j + 1] = 0;
                }
                break
            }
        }
    }

    digits
        .iter()
        .rev()
        .enumerate()
        .map(|(i, d)| 10u64.pow(i as u32) * d)
        .sum()
}


pub fn part_1(input: &Vec<Vec<u64>>) -> Option<u64> {
    let total:u64 = input.iter().map(|battery| find_highest_n(2, battery)).sum();
    Some(total)
}

pub fn part_2(input: &Vec<Vec<u64>>) -> Option<u64> {
    let total:u64 = input.iter().map(|battery| find_highest_n(12, battery)).sum();
    Some(total)
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_find_highest_n() {
        assert_eq!(find_highest_n(2,&vec![1,2,3,4,5]), 45);
        assert_eq!(find_highest_n(2,&vec![1,2,1,5,1]), 51);
        assert_eq!(find_highest_n(2, &vec![3,5,2,1,5]), 55);
        assert_eq!(find_highest_n(2, &vec![1,2,4,5,3]), 53);
        assert_eq!(find_highest_n(2, &vec![4,5,3,2,1]), 53);
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