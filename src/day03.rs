use crate::read_input_lines;
use anyhow::Result;
use itertools::Itertools;

pub fn prepare(file_name: &str) -> Result<Vec<String>> {
    let input = read_input_lines(file_name);
    Ok(input)
}

pub fn find_highest(batteries:&String) -> u32 {
    let digits: Vec<u32> = batteries
        .chars()
        .enumerate()
        .map(|(i, c)| (c.to_digit(10).unwrap(), i))
        .sorted()
        .rev()
        .take(2)
        .sorted_by_key(|k| k.1)
        .map(|(d, _)| d)
        .collect();

    // println!("{} {:?}", bank, digits);

    digits[0] * 10 + digits[1]
}

pub fn part_1(input: &Vec<String>) -> Option<u32> {
    let total:u32 = input.iter().map(find_highest).sum();
    Some(total)
}

pub fn part_2(_input: &Vec<String>) -> Option<usize> {
    None
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_find_highest() {
        assert_eq!(find_highest(&"12345".into()), 45);
        assert_eq!(find_highest(&"12151".into()), 25);
        assert_eq!(find_highest(&"35215".into()), 55);
        assert_eq!(find_highest(&"12453".into()), 45);
        assert_eq!(find_highest(&"45321".into()), 53);
    }

    #[test]
    fn test_part_1() {
        if let Ok(input) = prepare("day03-example.txt") {
            assert_eq!(part_1(&input), Some(357))
        }
    }

    #[test]
    #[ignore]
    fn test_part_2() {
        if let Ok(input) = prepare("day01-example.txt") {
            assert_eq!(part_2(&input), Some(1))
        }
    }
}